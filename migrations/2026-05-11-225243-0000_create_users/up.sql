CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	"username" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);