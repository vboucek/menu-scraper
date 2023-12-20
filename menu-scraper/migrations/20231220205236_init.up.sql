CREATE TABLE IF NOT EXISTS "User"
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username        TEXT NOT NULL UNIQUE,
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    password_salt   TEXT NOT NULL,
    profile_picture TEXT,
    deleted_at      TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "Group"
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    description TEXT,
    author_id   UUID NOT NULL,
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
    phone_number   TEXT,
    website        TEXT,
    email          TEXT,
    coordinates    POINT,
    monday_open    TSRANGE,
    tuesday_open   TSRANGE,
    wednesday_open TSRANGE,
    thursday_open  TSRANGE,
    friday_open    TSRANGE,
    saturday_open  TSRANGE,
    sunday_open    TSRANGE,
    lunch_server   TSRANGE,
    deleted_at     TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS "GroupUsers"
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL,
    group_id   UUID NOT NULL,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES "User" (id),
    FOREIGN KEY (group_id) REFERENCES "Group" (id)
);

CREATE TABLE IF NOT EXISTS "Lunch"
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date       DATE NOT NULL,
    group_id   UUID NOT NULL,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (group_id) REFERENCES "Group" (id)
);


CREATE TABLE IF NOT EXISTS "Vote"
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    restaurant_id UUID NOT NULL,
    user_id       UUID NOT NULL,
    lunch_id      UUID NOT NULL,
    deleted_at    TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES "User" (id),
    FOREIGN KEY (restaurant_id) REFERENCES "Restaurant" (id),
    FOREIGN KEY (lunch_id) REFERENCES "Lunch" (id)
);

CREATE TABLE IF NOT EXISTS "Menu"
(
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name          TEXT NOT NULL,
    price         INT  NOT NULL,
    size          TEXT,
    is_soup       BOOL NOT NULL,
    date          DATE NOT NULL,
    restaurant_id UUID NOT NULL,
    deleted_at    TIMESTAMPTZ,
    FOREIGN KEY (restaurant_id) REFERENCES "Restaurant" (id)
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

