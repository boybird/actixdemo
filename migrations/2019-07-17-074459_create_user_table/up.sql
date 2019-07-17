CREATE TABLE "public"."users" (
  "id" serial NOT NULL ,
  "username" varchar(50) unique,
  "password" varchar(50),
  "email" varchar(50) unique,
  "created_at" timestamp(6),
  "updated_at" timestamp(6),
  CONSTRAINT "users_pkey" PRIMARY KEY ("id")

);

