ALTER TABLE "users" ADD COLUMN "otp_enabled" BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE "otp_codes" (
    "id"         UUID        NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    "user_id"    UUID        NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "code"       VARCHAR     NOT NULL,
    "expires_at" TIMESTAMPTZ NOT NULL,
    "used"       BOOLEAN     NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
