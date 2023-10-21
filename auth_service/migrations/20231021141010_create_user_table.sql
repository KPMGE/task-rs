CREATE TABLE "users" (
  "id"          SERIAL NOT NULL,
  "name"        TEXT NOT NULL,
  "email"       TEXT NOT NULL,
  "created_at"  TIMESTAMP NOT NULL DEFAULT NOW(),
  "updated_at"  TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT "user_pkey" PRIMARY KEY("id")
)
