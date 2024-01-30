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
