ALTER TABLE "users" ADD COLUMN "is_verified" BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE "users" ADD COLUMN "verification_public_key" VARCHAR;
