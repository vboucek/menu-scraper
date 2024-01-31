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
    FOREIGN KEY (restaurant_id) REFERENCES "Restaurant" (id)
    );

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

INSERT INTO "User" (id, username, email, profile_picture, password_hash, deleted_at)
VALUES ('bfadb3a0-287c-4b5b-9132-cd977217a694', 'Jacky', 'jacky123@email.com', null, '123456789', null),
       ('c831db0d-23bf-4a88-8974-332fdea327cd', 'SpeedDemon', 'speederino@email.com', null, '123456789',
        null);

INSERT INTO "Group" (id, name, description, author_id, deleted_at)
VALUES ('4a51b8d6-c7dc-428b-bee6-97706063a0ae', 'Kámoši ze střední', '...', 'bfadb3a0-287c-4b5b-9132-cd977217a694',
        NULL);

INSERT INTO "Restaurant" (id, name, street, house_number, zip_code, city, picture)
VALUES ('7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', 'Pivnice Masný Růžek', 'Křenová', '70', '602 00', 'Brno',
        'temp_restaurant_img.png'),
       ('654669e4-3316-41eb-85f0-f6d1c619d840', 'U Karla', 'Bayerova', '578/8', '602 00', 'Brno',
        'temp_restaurant_2_img.png'),
       ('83db5c6c-e873-4b72-853a-9ddcfe4eb0a7', 'Plzeňský Dvůr', 'Šumavská', '29a', '602 00', 'Brno',
        'temp_restaurant_3_img.png');

INSERT INTO "Menu" (id, date, restaurant_id, deleted_at)
VALUES ('d528ed1d-bb13-4297-a760-f6e7692aa473', '2024-01-28', '7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', null),
       ('d704d684-f68b-487a-8062-4d1bb2b5797d', '2024-01-28', '654669e4-3316-41eb-85f0-f6d1c619d840', null),
       ('1bb8730d-ebba-465c-b52a-85071d944502', '2024-01-28', '7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', null),
       ('8b3cb051-e0ad-4a78-b840-fc71e24565e8', '2024-01-28', '654669e4-3316-41eb-85f0-f6d1c619d840', null),
       ('07b66746-9525-4e69-a2b1-9d15adf6fc23', '2024-01-28', '7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', null),
       ('389bd38c-3882-4260-8a16-d058f7e37ea3', '2024-01-29', '654669e4-3316-41eb-85f0-f6d1c619d840', null),
       ('fa8fa52c-83a6-43d0-9f05-2b2edc3106d8', '2024-01-29', '7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', null),
       ('e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2', '2024-01-30', '654669e4-3316-41eb-85f0-f6d1c619d840', null);


INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Boršč se zakysanou smetanou', 30, '0.25 l', true,
        'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('Rajská omáčka, masové kuličky, těstoviny', 120, '130g', false,
        'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('Kuřecí nudličky po Sečuánsku, opékané nudle', 145, '130g', false,
        'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('Španělský vepřový ptáček s uzeninou, vejcem, špekem a okurkem, divoká rýže, smažená cibulka',
        160, '160g', false, 'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('Filovaný hovězí flank steak se zeleninovým ratatouille, steakové hranolky, jalapeňos dip',
        200, '150g', false, 'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('Grilovaný losos marinovaný v citronové šťávě, kuskusový salát se sýrem Cottage a čerstvou zeleninou', 230,
        '150g', false, 'd528ed1d-bb13-4297-a760-f6e7692aa473');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Hovězí vývar se zeleninou, masem a nudlemi', 0, '', true,
        'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('Smažený kuřecí řízek s parmazánem, bramborový salát s hráškem, citrón', 175,
        '160g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('Špikovaná hovězí svíčková na smetaně, plátek citronu, brusinky, kynutý houskový knedlík', 175,
        '100g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('Quesadilla dvojitý sýr (cheddar, gouda, cherry rajčátka, limetková majonéza) a hranolky a dresink',
        175, '140g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('Grilovaný vepřový steak se slaninovou demi glace, šťouchané brambory s cibulkou, rajčatový salát', 175,
        '160g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('Špagety Carbonara (slanina, vejce, parmazán a listová petržel)', 135, '400g', false,
        'd704d684-f68b-487a-8062-4d1bb2b5797d');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Boršč se zakysanou smetanou', 30, '0.25 l', true,
        '1bb8730d-ebba-465c-b52a-85071d944502'),
       ('Rajská omáčka, masové kuličky, těstoviny', 120, '130g', false,
        '1bb8730d-ebba-465c-b52a-85071d944502'),
       ('Kuřecí nudličky po Sečuánsku, opékané nudle', 145, '130g', false,
        '1bb8730d-ebba-465c-b52a-85071d944502'),
       ('Španělský vepřový ptáček s uzeninou, vejcem, špekem a okurkem, divoká rýže, smažená cibulka',
        160, '160g', false, '1bb8730d-ebba-465c-b52a-85071d944502'),
       ('Filovaný hovězí flank steak se zeleninovým ratatouille, steakové hranolky, jalapeňos dip',
        200, '150g', false, '1bb8730d-ebba-465c-b52a-85071d944502'),
       ('Grilovaný losos marinovaný v citronové šťávě, kuskusový salát se sýrem Cottage a čerstvou zeleninou', 230,
        '150g', false, '1bb8730d-ebba-465c-b52a-85071d944502');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Hovězí vývar se zeleninou, masem a nudlemi', 0, '', true,
        '8b3cb051-e0ad-4a78-b840-fc71e24565e8'),
       ('Smažený kuřecí řízek s parmazánem, bramborový salát s hráškem, citrón', 175,
        '160g', false, '8b3cb051-e0ad-4a78-b840-fc71e24565e8'),
       ('Špikovaná hovězí svíčková na smetaně, plátek citronu, brusinky, kynutý houskový knedlík', 175,
        '100g', false, '8b3cb051-e0ad-4a78-b840-fc71e24565e8'),
       ('Quesadilla dvojitý sýr (cheddar, gouda, cherry rajčátka, limetková majonéza) a hranolky a dresink',
        175, '140g', false, '8b3cb051-e0ad-4a78-b840-fc71e24565e8'),
       ('Grilovaný vepřový steak se slaninovou demi glace, šťouchané brambory s cibulkou, rajčatový salát', 175,
        '160g', false, '8b3cb051-e0ad-4a78-b840-fc71e24565e8'),
       ('Špagety Carbonara (slanina, vejce, parmazán a listová petržel)', 135, '400g', false,
        '8b3cb051-e0ad-4a78-b840-fc71e24565e8');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Boršč se zakysanou smetanou', 30, '0.25 l', true,
        '07b66746-9525-4e69-a2b1-9d15adf6fc23'),
       ('Rajská omáčka, masové kuličky, těstoviny', 120, '130g', false,
        '07b66746-9525-4e69-a2b1-9d15adf6fc23'),
       ('Kuřecí nudličky po Sečuánsku, opékané nudle', 145, '130g', false,
        '07b66746-9525-4e69-a2b1-9d15adf6fc23'),
       ('Španělský vepřový ptáček s uzeninou, vejcem, špekem a okurkem, divoká rýže, smažená cibulka',
        160, '160g', false, '07b66746-9525-4e69-a2b1-9d15adf6fc23'),
       ('Filovaný hovězí flank steak se zeleninovým ratatouille, steakové hranolky, jalapeňos dip',
        200, '150g', false, '07b66746-9525-4e69-a2b1-9d15adf6fc23'),
       ('Grilovaný losos marinovaný v citronové šťávě, kuskusový salát se sýrem Cottage a čerstvou zeleninou', 230,
        '150g', false, '07b66746-9525-4e69-a2b1-9d15adf6fc23');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Hovězí vývar se zeleninou, masem a nudlemi', 0, '', true,
        '389bd38c-3882-4260-8a16-d058f7e37ea3'),
       ('Smažený kuřecí řízek s parmazánem, bramborový salát s hráškem, citrón', 175,
        '160g', false, '389bd38c-3882-4260-8a16-d058f7e37ea3'),
       ('Špikovaná hovězí svíčková na smetaně, plátek citronu, brusinky, kynutý houskový knedlík', 175,
        '100g', false, '389bd38c-3882-4260-8a16-d058f7e37ea3'),
       ('Quesadilla dvojitý sýr (cheddar, gouda, cherry rajčátka, limetková majonéza) a hranolky a dresink',
        175, '140g', false, '389bd38c-3882-4260-8a16-d058f7e37ea3'),
       ('Grilovaný vepřový steak se slaninovou demi glace, šťouchané brambory s cibulkou, rajčatový salát', 175,
        '160g', false, '389bd38c-3882-4260-8a16-d058f7e37ea3'),
       ('Špagety Carbonara (slanina, vejce, parmazán a listová petržel)', 135, '400g', false,
        '389bd38c-3882-4260-8a16-d058f7e37ea3');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Boršč se zakysanou smetanou', 30, '0.25 l', true,
        'fa8fa52c-83a6-43d0-9f05-2b2edc3106d8'),
       ('Rajská omáčka, masové kuličky, těstoviny', 120, '130g', false,
        'fa8fa52c-83a6-43d0-9f05-2b2edc3106d8'),
       ('Kuřecí nudličky po Sečuánsku, opékané nudle', 145, '130g', false,
        'fa8fa52c-83a6-43d0-9f05-2b2edc3106d8'),
       ('Španělský vepřový ptáček s uzeninou, vejcem, špekem a okurkem, divoká rýže, smažená cibulka',
        160, '160g', false, 'fa8fa52c-83a6-43d0-9f05-2b2edc3106d8'),
       ('Filovaný hovězí flank steak se zeleninovým ratatouille, steakové hranolky, jalapeňos dip',
        200, '150g', false, 'fa8fa52c-83a6-43d0-9f05-2b2edc3106d8'),
       ('Grilovaný losos marinovaný v citronové šťávě, kuskusový salát se sýrem Cottage a čerstvou zeleninou', 230,
        '150g', false, 'fa8fa52c-83a6-43d0-9f05-2b2edc3106d8');

INSERT INTO "MenuItem" (name, price, size, is_soup, menu_id)
VALUES ('Hovězí vývar se zeleninou, masem a nudlemi', 0, '', true,
        'e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2'),
       ('Smažený kuřecí řízek s parmazánem, bramborový salát s hráškem, citrón', 175,
        '160g', false, 'e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2'),
       ('Špikovaná hovězí svíčková na smetaně, plátek citronu, brusinky, kynutý houskový knedlík', 175,
        '100g', false, 'e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2'),
       ('Quesadilla dvojitý sýr (cheddar, gouda, cherry rajčátka, limetková majonéza) a hranolky a dresink',
        175, '140g', false, 'e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2'),
       ('Grilovaný vepřový steak se slaninovou demi glace, šťouchané brambory s cibulkou, rajčatový salát', 175,
        '160g', false, 'e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2'),
       ('Špagety Carbonara (slanina, vejce, parmazán a listová petržel)', 135, '400g', false,
        'e1bbb6a5-ba5b-47e7-ad2e-5bcadc5b5ef2');


INSERT INTO "GroupUsers" (id, user_id, group_id, deleted_at)
VALUES ('b12a7839-4b1f-427d-9acd-d0d6eb8c39f0', 'c831db0d-23bf-4a88-8974-332fdea327cd',
        '4a51b8d6-c7dc-428b-bee6-97706063a0ae', null);

INSERT INTO "Lunch" (id, date, group_id, deleted_at)
VALUES ('645ae55a-190e-4b5d-b47b-0c00c9f4ce0d', '2024-01-15', '4a51b8d6-c7dc-428b-bee6-97706063a0ae', null);

INSERT INTO "Vote" (id, menu_id, user_id, lunch_id, deleted_at)
VALUES ('80c6b27a-4ed0-4ed3-8a79-ed6f49edc475', 'd528ed1d-bb13-4297-a760-f6e7692aa473',
        'bfadb3a0-287c-4b5b-9132-cd977217a694', '645ae55a-190e-4b5d-b47b-0c00c9f4ce0d', null),
       ('2b107d66-fe72-400d-8df0-061ca27242dd', 'd704d684-f68b-487a-8062-4d1bb2b5797d',
        'c831db0d-23bf-4a88-8974-332fdea327cd', '645ae55a-190e-4b5d-b47b-0c00c9f4ce0d', null);