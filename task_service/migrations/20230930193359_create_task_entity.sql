CREATE TABLE "tasks" (
  "id"          SERIAL NOT NULL,
  "title"       TEXT NOT NULL,
  "description" TEXT NOT NULL DEFAULT '',
  "due_date"    TIMESTAMP,
  "created_at"  TIMESTAMP NOT NULL DEFAULT NOW(),
  "updated_at"  TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT "task_pkey" PRIMARY KEY("id")
);
