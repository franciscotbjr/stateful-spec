// The legal state transitions of the multi-agent flow, as pure functions.
// Each verb validates its precondition and, on success, mutates the frontmatter
// and bumps the monotonic `seq`. Illegal transitions throw, leaving state untouched.

import { FrontMatter } from "./frontmatter.js";

export type Verb =
  | { kind: "spec-ready" }
  | { kind: "approve" }
  | { kind: "request-changes"; reason: string }
  | { kind: "block"; reason: string }
  | { kind: "submit" }
  | { kind: "advance" }
  | { kind: "approve-plan"; total: number }
  | { kind: "submit-spec" }
  | { kind: "approve-spec" }
  | { kind: "request-spec-changes"; reason: string }
  | { kind: "hand-back"; reason: string };

export interface Outcome {
  role: string;
  action: string;
}

export function apply(fm: FrontMatter, verb: Verb): Outcome {
  const status = fm.get("process_status") ?? "";
  const turn = fm.get("turn") ?? "";
  const step = fm.get("step") ?? "";

  let outcome: Outcome;
  switch (verb.kind) {
    case "spec-ready": {
      requireRunning(status);
      requireTurn(turn, "pm");
      requireStep(step, ["SPEC_APPROVED"]);
      fm.set("step", "SPEC_READY");
      fm.set("turn", "engineer");
      fm.set("updated_by", "pm");
      outcome = { role: "pm", action: "SPEC_APPROVED→SPEC_READY (handoff to engineer)" };
      break;
    }
    case "submit-spec": {
      requireRunning(status);
      requireTurn(turn, "pm");
      requireStep(step, ["SPEC_PENDING", "SPEC_CHANGES_REQUESTED"]);
      fm.set("step", "ARCH_REVIEW");
      fm.set("turn", "architect");
      fm.set("updated_by", "pm");
      outcome = { role: "pm", action: `${step}→ARCH_REVIEW (spec submitted to reviewer)` };
      break;
    }
    case "approve-spec": {
      requireRunning(status);
      requireTurn(turn, "architect");
      requireStep(step, ["ARCH_REVIEW"]);
      fm.set("step", "SPEC_APPROVED");
      fm.set("turn", "pm");
      fm.set("updated_by", "architect");
      outcome = { role: "architect", action: "ARCH_REVIEW→SPEC_APPROVED (design gate approved)" };
      break;
    }
    case "request-spec-changes": {
      requireRunning(status);
      requireTurn(turn, "architect");
      requireStep(step, ["ARCH_REVIEW"]);
      const round = fm.getU64("spec_review_round") + 1;
      const max = fm.getU64("max_spec_review_rounds");
      fm.set("spec_review_round", String(round));
      fm.set("step", "SPEC_CHANGES_REQUESTED");
      fm.set("turn", "pm");
      fm.set("updated_by", "architect");
      if (max > 0 && round > max) {
        fm.set("process_status", "BLOCKED");
        fm.set("blocked_reason", `stall: spec_review_round ${round} > max ${max} — ${verb.reason}`);
        outcome = {
          role: "architect",
          action: `ARCH_REVIEW→SPEC_CHANGES_REQUESTED (spec round ${round}) → BLOCKED (stall)`,
        };
      } else {
        outcome = {
          role: "architect",
          action: `ARCH_REVIEW→SPEC_CHANGES_REQUESTED (spec round ${round}): ${verb.reason}`,
        };
      }
      break;
    }
    case "hand-back": {
      requireRunning(status);
      requireTurn(turn, "engineer");
      requireStep(step, ["SPEC_READY", "CHANGES_REQUESTED"]);
      fm.set("step", "SPEC_PENDING");
      fm.set("turn", "pm");
      fm.set("spec_review_round", "0");
      fm.set("updated_by", "engineer");
      outcome = {
        role: "engineer",
        action: `${step}→SPEC_PENDING (handed back to PM for re-scope): ${verb.reason}`,
      };
      break;
    }
    case "approve": {
      requireRunning(status);
      requireTurn(turn, "pm");
      requireStep(step, ["AWAITING_REVIEW"]);
      fm.set("step", "APPROVED");
      fm.set("turn", "engineer");
      fm.set("updated_by", "pm");
      outcome = { role: "pm", action: "AWAITING_REVIEW→APPROVED" };
      break;
    }
    case "request-changes": {
      requireRunning(status);
      requireTurn(turn, "pm");
      requireStep(step, ["AWAITING_REVIEW"]);
      const round = fm.getU64("review_round") + 1;
      const max = fm.getU64("max_review_rounds");
      fm.set("review_round", String(round));
      fm.set("step", "CHANGES_REQUESTED");
      fm.set("turn", "engineer");
      fm.set("updated_by", "pm");
      if (max > 0 && round > max) {
        fm.set("process_status", "BLOCKED");
        fm.set("blocked_reason", `stall: review_round ${round} > max ${max} — ${verb.reason}`);
        outcome = {
          role: "pm",
          action: `AWAITING_REVIEW→CHANGES_REQUESTED (round ${round}) → BLOCKED (stall)`,
        };
      } else {
        outcome = {
          role: "pm",
          action: `AWAITING_REVIEW→CHANGES_REQUESTED (round ${round}): ${verb.reason}`,
        };
      }
      break;
    }
    case "block": {
      fm.set("process_status", "BLOCKED");
      fm.set("blocked_reason", verb.reason);
      const role = turn === "engineer" ? "engineer" : "pm";
      fm.set("updated_by", role);
      outcome = { role, action: `→BLOCKED: ${verb.reason}` };
      break;
    }
    case "submit": {
      requireRunning(status);
      requireTurn(turn, "engineer");
      requireStep(step, ["SPEC_READY", "CHANGES_REQUESTED"]);
      fm.set("step", "AWAITING_REVIEW");
      fm.set("turn", "pm");
      fm.set("updated_by", "engineer");
      outcome = { role: "engineer", action: `${step}→AWAITING_REVIEW (delivery; gate green)` };
      break;
    }
    case "advance": {
      requireRunning(status);
      requireTurn(turn, "engineer");
      requireStep(step, ["APPROVED"]);
      const cur = fm.getU64("current_milestone");
      const total = fm.getU64("total_milestones");
      fm.set("updated_by", "engineer");
      if (total > 0 && cur >= total) {
        fm.set("process_status", "DONE");
        fm.set("done", "true");
        fm.set("step", "COMMITTED");
        fm.set("turn", "pm");
        outcome = { role: "engineer", action: `APPROVED→COMMITTED (m${cur}) → DONE (last milestone)` };
      } else {
        const nextM = cur + 1;
        fm.set("current_milestone", String(nextM));
        fm.set("step", "SPEC_PENDING");
        fm.set("review_round", "0");
        fm.set("spec_review_round", "0");
        fm.set("turn", "pm");
        outcome = { role: "engineer", action: `APPROVED→COMMITTED (m${cur}) → m${nextM} SPEC_PENDING` };
      }
      break;
    }
    case "approve-plan": {
      if (status !== "AWAITING_PLAN_APPROVAL") {
        throw new Error(`process_status must be AWAITING_PLAN_APPROVAL (found '${status}')`);
      }
      requireTurn(turn, "pm");
      fm.set("process_status", "RUNNING");
      fm.set("total_milestones", String(verb.total));
      fm.set("current_milestone", "1");
      fm.set("step", "SPEC_PENDING");
      fm.set("review_round", "0");
      fm.set("spec_review_round", "0");
      fm.set("turn", "pm");
      fm.set("updated_by", "pm");
      outcome = {
        role: "pm",
        action: `AWAITING_PLAN_APPROVAL→RUNNING (plan approved; total=${verb.total}; m1 SPEC_PENDING)`,
      };
      break;
    }
  }

  fm.bumpSeq();
  return outcome;
}

function requireRunning(status: string): void {
  if (status !== "RUNNING") {
    throw new Error(`process_status must be RUNNING (found '${status}')`);
  }
}

function requireTurn(turn: string, expected: string): void {
  if (turn !== expected) {
    throw new Error(`not ${expected}'s turn (turn='${turn}')`);
  }
}

function requireStep(step: string, allowed: string[]): void {
  if (!allowed.includes(step)) {
    throw new Error(`step must be one of ${JSON.stringify(allowed)} (found '${step}')`);
  }
}
