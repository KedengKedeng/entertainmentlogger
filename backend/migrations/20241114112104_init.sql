    -- CreateTable
    CREATE TABLE IF NOT EXISTS "users" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "last_online" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "email" TEXT NOT NULL,
        "password" TEXT NOT NULL,
        "username" TEXT NOT NULL,
        "profile_picture" TEXT,
        "bio" TEXT NOT NULL DEFAULT '',

        CONSTRAINT "users_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "media_to_user_link" (
        "id" TEXT NOT NULL,
        "date_added" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "date_started" TIMESTAMPTZ(3),
        "date_ended" TIMESTAMPTZ(3),
        "status_type" INTEGER NOT NULL,
        "rating" INTEGER,
        "scenes_seen" INTEGER NOT NULL DEFAULT 0,
        "acts_seen" INTEGER NOT NULL DEFAULT 0,
        "media_id" TEXT NOT NULL,
        "user_id" TEXT NOT NULL,

        CONSTRAINT "media_to_user_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "franchise_to_user_link" (
        "id" TEXT NOT NULL,
        "date_added" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "user_id" TEXT NOT NULL,
        "franchise_id" TEXT NOT NULL,

        CONSTRAINT "franchise_to_user_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "franchises" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "edited" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "first_showing" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "picture" TEXT,
        "name" TEXT NOT NULL,
        "bio" TEXT NOT NULL DEFAULT '',

        CONSTRAINT "franchises_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "franchise_to_media_link" (
        "id" TEXT NOT NULL,
        "franchise_id" TEXT NOT NULL,
        "media_id" TEXT NOT NULL,

        CONSTRAINT "franchise_to_media_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "media" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "edited" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "release_date" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "end_date" TIMESTAMPTZ(3) DEFAULT CURRENT_TIMESTAMP,
        "media_type" INTEGER NOT NULL,
        "picture" TEXT,
        "name" TEXT NOT NULL,
        "bio" TEXT NOT NULL DEFAULT '',
        "scene_count" INTEGER NOT NULL,
        "act_count" INTEGER NOT NULL,

        CONSTRAINT "media_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "genre_to_media_link" (
        "id" TEXT NOT NULL,
        "genre_id" TEXT NOT NULL,
        "media_id" TEXT NOT NULL,

        CONSTRAINT "genre_to_media_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "genres" (
        "name" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

        CONSTRAINT "genres_pkey" PRIMARY KEY ("name")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "people" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "name" TEXT NOT NULL,
        "picture" TEXT,
        "bio" TEXT NOT NULL DEFAULT '',

        CONSTRAINT "people_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "person_to_media_link" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "role" TEXT NOT NULL,
        "person_id" TEXT NOT NULL,
        "media_id" TEXT NOT NULL,

        CONSTRAINT "person_to_media_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "person_to_company_link" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "from" TIMESTAMPTZ(3) NOT NULL,
        "to" TIMESTAMPTZ(3),
        "person_id" TEXT NOT NULL,
        "company_id" TEXT NOT NULL,

        CONSTRAINT "person_to_company_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "companies" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "name" TEXT NOT NULL,
        "picture" TEXT,
        "bio" TEXT NOT NULL DEFAULT '',

        CONSTRAINT "companies_pkey" PRIMARY KEY ("id")
    );

    -- CreateTable
    CREATE TABLE IF NOT EXISTS "company_to_media_link" (
        "id" TEXT NOT NULL,
        "created" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "role" TEXT NOT NULL,
        "company_id" TEXT NOT NULL,
        "media_id" TEXT NOT NULL,

        CONSTRAINT "company_to_media_link_pkey" PRIMARY KEY ("id")
    );

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "users_email_key" ON "users"("email");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "users_password_key" ON "users"("password");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "media_to_user_link_user_id_media_id_key" ON "media_to_user_link"("user_id", "media_id");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "franchise_to_user_link_user_id_franchise_id_key" ON "franchise_to_user_link"("user_id", "franchise_id");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "franchise_to_media_link_franchise_id_media_id_key" ON "franchise_to_media_link"("franchise_id", "media_id");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "genre_to_media_link_genre_id_media_id_key" ON "genre_to_media_link"("genre_id", "media_id");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "person_to_media_link_person_id_media_id_key" ON "person_to_media_link"("person_id", "media_id");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "person_to_company_link_person_id_company_id_key" ON "person_to_company_link"("person_id", "company_id");

    -- CreateIndex
    CREATE UNIQUE INDEX IF NOT EXISTS "company_to_media_link_company_id_media_id_key" ON "company_to_media_link"("company_id", "media_id");

DO $$
BEGIN
    -- AddForeignKey
    ALTER TABLE "media_to_user_link" ADD CONSTRAINT "media_to_user_link_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "media"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "media_to_user_link" ADD CONSTRAINT "media_to_user_link_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "franchise_to_user_link" ADD CONSTRAINT "franchise_to_user_link_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "franchise_to_user_link" ADD CONSTRAINT "franchise_to_user_link_franchise_id_fkey" FOREIGN KEY ("franchise_id") REFERENCES "franchises"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "franchise_to_media_link" ADD CONSTRAINT "franchise_to_media_link_franchise_id_fkey" FOREIGN KEY ("franchise_id") REFERENCES "franchises"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "franchise_to_media_link" ADD CONSTRAINT "franchise_to_media_link_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "media"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "genre_to_media_link" ADD CONSTRAINT "genre_to_media_link_genre_id_fkey" FOREIGN KEY ("genre_id") REFERENCES "genres"("name") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "genre_to_media_link" ADD CONSTRAINT "genre_to_media_link_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "media"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "person_to_media_link" ADD CONSTRAINT "person_to_media_link_person_id_fkey" FOREIGN KEY ("person_id") REFERENCES "people"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "person_to_media_link" ADD CONSTRAINT "person_to_media_link_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "media"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "person_to_company_link" ADD CONSTRAINT "person_to_company_link_person_id_fkey" FOREIGN KEY ("person_id") REFERENCES "people"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "person_to_company_link" ADD CONSTRAINT "person_to_company_link_company_id_fkey" FOREIGN KEY ("company_id") REFERENCES "companies"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "company_to_media_link" ADD CONSTRAINT "company_to_media_link_company_id_fkey" FOREIGN KEY ("company_id") REFERENCES "companies"("id") ON DELETE CASCADE ON UPDATE CASCADE;

    -- AddForeignKey
    ALTER TABLE "company_to_media_link" ADD CONSTRAINT "company_to_media_link_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "media"("id") ON DELETE CASCADE ON UPDATE CASCADE;
EXCEPTION WHEN duplicate_object THEN
    -- If any of these are a duplicate, all of them are, so skip everything
END;
$$;