INSERT INTO "User" (id, username, email, profile_picture, password_hash, password_salt, deleted_at)
VALUES ('bfadb3a0-287c-4b5b-9132-cd977217a694', 'Jacky', 'jacky123@email.com', null, '123456789', '123456789', null),
       ('c831db0d-23bf-4a88-8974-332fdea327cd', 'SpeedDemon', 'speederino@email.com', null, '123456789', '123456789',
        null);

INSERT INTO "Group" (id, name, description, author_id, deleted_at)
VALUES ('4a51b8d6-c7dc-428b-bee6-97706063a0ae', 'Kámoši ze střední', '...', 'bfadb3a0-287c-4b5b-9132-cd977217a694',
        NULL);

INSERT INTO "Restaurant" (id, name, street, house_number, zip_code, city)
VALUES ('7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', 'Pivnice Masný Růžek', 'Křenová', '70', '602 00', 'Brno'),
       ('654669e4-3316-41eb-85f0-f6d1c619d840', 'U Karla', 'Bayerova', '578/8', '602 00', 'Brno'),
       ('83db5c6c-e873-4b72-853a-9ddcfe4eb0a7', 'Plzeňský Dvůr', 'Šumavská', '29a', '602 00', 'Brno');

INSERT INTO "Menu" (id, date, restaurant_id, deleted_at)
VALUES ('d528ed1d-bb13-4297-a760-f6e7692aa473', '2024-01-15', '7d7ec998-45da-41ee-bb4c-ac5bbe0e4669', null),
       ('d704d684-f68b-487a-8062-4d1bb2b5797d', '2024-01-15', '654669e4-3316-41eb-85f0-f6d1c619d840', null);

INSERT INTO "MenuItem" (id, name, price, size, is_soup, menu_id)
VALUES ('d4790158-a75d-4dd2-9f9c-d3819352d1fd', 'Boršč se zakysanou smetanou', 30, '0.25 l', true,
        'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('57093880-9075-402e-be2f-439e7c4b5bbe', 'Rajská omáčka, masové kuličky, těstoviny', 120, '130g', false,
        'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('d32bfb20-d95e-4f1e-a8e5-6d4480336be1', 'Kuřecí nudličky po Sečuánsku, opékané nudle', 145, '130g', false,
        'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('8e4d2c85-8f34-4a5e-a5d8-1998bcadb1a0',
        'Španělský vepřový ptáček s uzeninou, vejcem, špekem a okurkem, divoká rýže, smažená cibulka',
        160, '160g', false, 'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('2871fd93-d2f6-4463-94be-99acf8d09d8e',
        'Filovaný hovězí flank steak se zeleninovým ratatouille, steakové hranolky, jalapeňos dip',
        200, '150g', false, 'd528ed1d-bb13-4297-a760-f6e7692aa473'),
       ('ffeb1e7e-417b-41ca-8638-079f610b3bd5',
        'Grilovaný losos marinovaný v citronové šťávě, kuskusový salát se sýrem Cottage a čerstvou zeleninou', 230,
        '150g', false, 'd528ed1d-bb13-4297-a760-f6e7692aa473');

INSERT INTO "MenuItem" (id, name, price, size, is_soup, menu_id)
VALUES ('79c4d79a-303e-4a95-b1af-e8203a3d3b14', 'Hovězí vývar se zeleninou, masem a nudlemi', 0, '', true,
        'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('d81b7122-c532-4137-9522-10eb83cd1c32',
        'Smažený kuřecí řízek s parmazánem, bramborový salát s hráškem, citrón', 175,
        '160g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('a9b0ad1e-244b-418c-af7e-6f828558c4d8',
        'Špikovaná hovězí svíčková na smetaně, plátek citronu, brusinky, kynutý houskový knedlík', 175,
        '100g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('0bb1199c-d7c9-46ec-b761-a10edd7e01e4',
        'Quesadilla dvojitý sýr (cheddar, gouda, cherry rajčátka, limetková majonéza) a hranolky a dresink',
        175, '140g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('5fe60c96-fe09-4302-8274-acfa4f9bb44f',
        'Grilovaný vepřový steak se slaninovou demi glace, šťouchané brambory s cibulkou, rajčatový salát', 175,
        '160g', false, 'd704d684-f68b-487a-8062-4d1bb2b5797d'),
       ('e1ebf3db-b197-45b4-ba52-f734b60e6fc5',
        'Špagety Carbonara (slanina, vejce, parmazán a listová petržel)', 135, '400g', false,
        'd704d684-f68b-487a-8062-4d1bb2b5797d');


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