-- Required for calculating distance between points
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS "User"
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username        TEXT NOT NULL UNIQUE,
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    profile_picture TEXT,
    deleted_at      TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "Group"
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    description TEXT,
    author_id   UUID NOT NULL,
    picture     TEXT,
    deleted_at  TIMESTAMPTZ,
    FOREIGN KEY (author_id) REFERENCES "User" (id)
);

CREATE TABLE IF NOT EXISTS "Restaurant"
(
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name           TEXT NOT NULL,
    street         TEXT NOT NULL,
    house_number   TEXT NOT NULL,
    zip_code       TEXT NOT NULL,
    city           TEXT NOT NULL,
    picture        TEXT,
    phone_number   TEXT,
    website        TEXT,
    email          TEXT,
    coordinates    POINT,
    monday_open    TEXT,
    tuesday_open   TEXT,
    wednesday_open TEXT,
    thursday_open  TEXT,
    friday_open    TEXT,
    saturday_open  TEXT,
    sunday_open    TEXT,
    lunch_served   TEXT,
    deleted_at     TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "GroupUsers"
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL,
    group_id   UUID NOT NULL,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES "User" (id),
    FOREIGN KEY (group_id) REFERENCES "Group" (id),
    CONSTRAINT user_group_unique UNIQUE (user_id, group_id)
);

CREATE TABLE IF NOT EXISTS "Lunch"
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date       DATE NOT NULL,
    group_id   UUID NOT NULL,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (group_id) REFERENCES "Group" (id),
    CONSTRAINT group_date UNIQUE (date, group_id)
);

CREATE TABLE IF NOT EXISTS "Menu"
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date          DATE NOT NULL,
    restaurant_id UUID NOT NULL,
    deleted_at    TIMESTAMPTZ,
    FOREIGN KEY (restaurant_id) REFERENCES "Restaurant" (id),
    CONSTRAINT restaurant_date_unique UNIQUE (date, restaurant_id)
);

CREATE UNIQUE INDEX date_restaurant on "Menu" (date, restaurant_id);

CREATE TABLE IF NOT EXISTS "MenuItem"
(
    id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name    TEXT NOT NULL,
    price   INT  NOT NULL,
    size    TEXT NOT NULL,
    is_soup BOOL NOT NULL,
    menu_id UUID NOT NULL,
    FOREIGN KEY (menu_id) REFERENCES "Menu" (id)
);

CREATE TABLE IF NOT EXISTS "Vote"
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    menu_id    UUID NOT NULL,
    user_id    UUID NOT NULL,
    lunch_id   UUID NOT NULL,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES "User" (id),
    FOREIGN KEY (menu_id) REFERENCES "Menu" (id),
    FOREIGN KEY (lunch_id) REFERENCES "Lunch" (id),
    CONSTRAINT user_vote UNIQUE (user_id, lunch_id)
);

CREATE TABLE IF NOT EXISTS "Tag"
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "RestaurantTags"
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tag_id        UUID NOT NULL,
    restaurant_id UUID NOT NULL,
    deleted_at    TIMESTAMPTZ,
    FOREIGN KEY (tag_id) REFERENCES "Tag" (id),
    FOREIGN KEY (restaurant_id) REFERENCES "Restaurant" (id)
);

-- Some mock data
INSERT INTO "User" (id, username, email, profile_picture, password_hash, deleted_at)
VALUES ('a1259611-9130-421e-8533-fa26ed56c5f0', 'Cheems', 'cheems@gmail.com',
        '5d639177-9645-427f-b10f-85d30e470ef7.jpeg',
        '$argon2id$v=19$m=19456,t=2,p=1$wvPDHiK6hTDIyHiiNRHJkg$j79lVRxsp0oLMFuiRHfNHZZLgfV8dSegx9E25DBFDWc', null),
       ('2cdf114c-df6a-46d2-9388-4d7c3ed451fc', 'UncleMurphy', 'unclemurphy@seznam.cz',
        '765038aa-6e9f-4a69-86bc-bfd80e401f3f.png',
        '$argon2id$v=19$m=19456,t=2,p=1$IT7aqFOS325WZ5DBrkqmgA$z3fnGiTnvptBdad60HxngZj5WO5TDCXpsQHmIHga1Tc', null),
       ('23168877-bb6f-4d28-bfc2-eabc0e9c25a1', 'Doge', 'doge@outlook.com', '0c76029b-b83b-49c9-b5f6-02e4eeb8f898.jpeg',
        '$argon2id$v=19$m=19456,t=2,p=1$JITm74C53o+REcCGxnm03Q$bUwIxc1K2zQrOaMsCyaRmTHZ/ZLKR+gyldzqr6tzlh0', null),
       ('5ad84f3f-4a6a-4b12-ac50-d050c5e8c0ba', 'ComradeDoggo', 'comradedoggo@gmail.com',
        '31a1e740-9a06-4e28-a4d2-65aaa502c36e.png',
        '$argon2id$v=19$m=19456,t=2,p=1$rX+fCn3sc+aZET5PWJWjIQ$pN6OCHJQQrvK1Xd7ImLSN2LJFQsNUPMLkf9DsrYd6N4', null),
       ('314a6e5f-5c9f-42b6-8abd-6b161c08d9aa', 'Walter', 'walter@seznam.cz',
        '321e1692-ecd3-407d-bb01-cffa68f154e2.jpeg',
        '$argon2id$v=19$m=19456,t=2,p=1$JFMbBSczPIv41FKqlxg+cg$bmEyoBDEkdav+BKAeVkdDlInpCOtmNKLfqEoLwL2uhw', null);

INSERT INTO "Group" (id, name, description, author_id, picture, deleted_at)
VALUES ('2ee2b0bc-cf05-4221-a723-be5ea30acafd', 'Doggos', 'Doggos, all kinds, all colours and breeds.',
        'a1259611-9130-421e-8533-fa26ed56c5f0',
        '506dffc4-ce5c-4769-b068-f0bc55ae9b9c.jpeg', NULL);

INSERT INTO "GroupUsers" (id, user_id, group_id, deleted_at)
VALUES ('32889264-28b6-4061-91eb-8ebe91bab8fb', '23168877-bb6f-4d28-bfc2-eabc0e9c25a1',
        '2ee2b0bc-cf05-4221-a723-be5ea30acafd', NULL),
       ('cada2e7b-e025-4b22-bc2d-fd6b415188b7', '5ad84f3f-4a6a-4b12-ac50-d050c5e8c0ba',
        '2ee2b0bc-cf05-4221-a723-be5ea30acafd', NULL),
       ('ee0bd397-a43e-4e08-89fc-3fea02697038', '314a6e5f-5c9f-42b6-8abd-6b161c08d9aa',
        '2ee2b0bc-cf05-4221-a723-be5ea30acafd', NULL),
       ('0f315dbc-13d9-4d69-9fc1-4575d4019c8a', '2cdf114c-df6a-46d2-9388-4d7c3ed451fc',
        '2ee2b0bc-cf05-4221-a723-be5ea30acafd', NULL);

