CREATE TABLE tenant_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL UNIQUE REFERENCES tenants(id) ON DELETE CASCADE,
    allow_signups BOOLEAN DEFAULT true,
    require_email_verification BOOLEAN DEFAULT true,
    enable_2fa BOOLEAN DEFAULT false,
    session_duration_minutes INTEGER DEFAULT 1440,
    allowed_oauth_providers TEXT[] DEFAULT '{}',
    custom_email_templates JSONB DEFAULT '{}',
    webhook_url VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_tenant_settings_tenant_id ON tenant_settings(tenant_id);
