# Endpoint Specification

> Use this template for API endpoints (REST, GraphQL, gRPC, WebSocket).

---

## Metadata

- **Endpoint:** [METHOD /path — e.g., POST /api/users]
- **Protocol:** REST | GraphQL | gRPC | WebSocket
- **Status:** Draft | Ready | In Progress | Complete
- **Complexity:** Simple | Medium | Complex
- **Authentication:** [e.g., Bearer token, API key, none]

## Description

[What this endpoint does, in 1-2 sentences.]

## Request

### Parameters

#### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| [name] | [type] | Yes/No | [description] |

#### Query Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| [name] | [type] | Yes/No | [value] | [description] |

#### Headers

| Header | Required | Description |
|--------|----------|-------------|
| [name] | Yes/No | [description] |

### Request Body

```json
{
  "field": "value",
  "optional_field": "value"
}
```

| Field | Type | Required | Default | Validation | Description |
|-------|------|----------|---------|------------|-------------|
| [name] | [type] | Yes/No | [value] | [rules] | [description] |

## Response

### Success Response

**Status:** [e.g., 200 OK, 201 Created]

```json
{
  "field": "value"
}
```

| Field | Type | Description |
|-------|------|-------------|
| [name] | [type] | [description] |

### Error Responses

| Status | Condition | Response Body |
|--------|-----------|---------------|
| 400 | [Invalid input] | `{"error": "description"}` |
| 401 | [Unauthorized] | `{"error": "description"}` |
| 404 | [Not found] | `{"error": "description"}` |
| 500 | [Server error] | `{"error": "description"}` |

## Validation Rules

- [e.g., "field_name must be non-empty"]
- [e.g., "email must be a valid email format"]
- [e.g., "limit must be between 1 and 100"]

## Test Scenarios

### Happy Path

| # | Scenario | Request | Expected Status | Expected Body |
|---|----------|---------|-----------------|---------------|
| 1 | [Normal request] | [summary] | [200] | [summary] |

### Edge Cases

| # | Scenario | Request | Expected Status | Expected Body |
|---|----------|---------|-----------------|---------------|
| 1 | [Boundary input] | [summary] | [status] | [summary] |

### Error Cases

| # | Scenario | Request | Expected Status | Expected Error |
|---|----------|---------|-----------------|----------------|
| 1 | [Missing required field] | [summary] | [400] | [error message] |

## Dependencies

- **Requires:** [Database tables, services, middleware]
- **Impacts:** [Other endpoints, webhooks, caches]
- **External:** [Third-party APIs called by this endpoint]

## Implementation Notes

[Suggested approach, performance considerations, caching strategy, rate limiting, pagination details, etc.]

## Examples

### cURL

```bash
curl -X POST https://api.example.com/endpoint \
  -H "Content-Type: application/json" \
  -d '{"field": "value"}'
```

### Code

```
[Example in the project's primary language showing how to call this endpoint]
```

## References

- [Link to related spec, API documentation, or external API reference]
