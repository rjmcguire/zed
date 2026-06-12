# Enterprise Features

Zed's enterprise features provide organizations with enhanced control, visibility, and compliance capabilities.

## Enterprise Settings

Enterprise features can be configured via the `enterprise` block in `settings.json`.

### System-wide Configuration

Administrators can enforce settings across all users by creating a `settings.json` file in the system-wide configuration directory:

- **Windows**: `C:\ProgramData\Zed\settings.json`
- **Linux**: `/etc/zed/settings.json`

These settings will be loaded as "System" settings and take **highest precedence**. They will override all other settings, including an individual user's `settings.json` and even project-local `.zed/settings.json` files. This ensures that organization policies are strictly enforced across the entire application.

### Configuration in settings.json

```json
{
  "enterprise": {
    "enabled": true,
    "organization_id": "your-organization-id",
    "allowed_extensions": ["extension-id-1", "extension-id-2"],
    "allowed_ai_providers": ["provider-1"],
    "audit_url": "https://your-audit-endpoint.com/events"
  }
}
```

### Configuration Fields

- **enabled**: (boolean) Master switch for enterprise features.
- **organization_id**: (string) Unique identifier for your organization. This ID is included in audit logs and telemetry events.
- **allowed_extensions**: (array of strings) Optional list of allowed extension IDs. If specified, installations of extensions not in this list will be blocked.
- **allowed_ai_providers**: (array of strings) Optional list of allowed AI providers.
- **audit_url**: (string) URL where audit events (e.g., blocked extension installations) are reported via POST requests.

## Audit Logging

When `enterprise.enabled` is true and an extension installation is blocked by the `allowed_extensions` policy, Zed reports this event to the configured `audit_url`.

### Blocked Extension Event Payload

The event is sent as a JSON POST request:

```json
{
  "event": "extension_install_blocked",
  "extension_id": "blocked-extension-id",
  "organization_id": "your-organization-id"
}
```

## Telemetry

When enterprise features are enabled, all telemetry events flushed to Zed's servers include the `organization_id`. This allows for organizational-level reporting and analysis.

### Telemetry Request Structure

Telemetry event batches include the `organization_id` in the request body:

```json
{
  "events": [...],
  "organization_id": "your-organization-id",
  ...
}
```

This ensures that all activity from an organization can be correctly attributed and analyzed within their specific context.
