CREATE TABLE "public"."users" (
  "id" serial NOT NULL ,
  "username" varchar(50) COLLATE "pg_catalog"."default",
  "password" varchar(50) COLLATE "pg_catalog"."default",
  "email" varchar(50) COLLATE "pg_catalog"."default",
  "created_at" timestamp(6),
  "updated_at" timestamp(6),
  CONSTRAINT "users_pkey" PRIMARY KEY ("id")

);

