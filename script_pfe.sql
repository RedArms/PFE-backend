DROP SCHEMA IF EXISTS pfe CASCADE;
CREATE SCHEMA pfe;

CREATE SCHEMA IF NOT EXISTS pfe;

-- Users
CREATE TABLE pfe.users (
                           user_id SERIAL PRIMARY KEY,
                           first_name VARCHAR(255) NOT NULL,
                           last_name VARCHAR(255) NOT NULL,
                           email VARCHAR(255) NOT NULL UNIQUE,
                           phone VARCHAR(255) NOT NULL UNIQUE,
                           password VARCHAR(255) NOT NULL,
                           is_admin BOOLEAN NOT NULL DEFAULT false,
                           is_delivery_person BOOLEAN NOT NULL DEFAULT false,
                           is_verified BOOLEAN NOT NULL DEFAULT false
);

-- Tours
CREATE TABLE pfe.tours (
                           tour_id SERIAL PRIMARY KEY,
                           geo_zone VARCHAR(255) NOT NULL,
                           delivery_person INTEGER REFERENCES pfe.users(user_id) NOT NULL
);

-- Clients
CREATE TABLE pfe.clients (
                             client_id SERIAL PRIMARY KEY,
                             name VARCHAR(255) NOT NULL,
                             address VARCHAR(255) NOT NULL,
                             tour INTEGER REFERENCES pfe.tours(tour_id)
);

-- Items
CREATE TABLE pfe.items (
                           item_id SERIAL PRIMARY KEY,
                           label VARCHAR(255) NOT NULL,
                           size VARCHAR(3) NULL CHECK (size IN ('S', 'M', 'L', 'XL'))
);

-- Client Lines
CREATE TABLE pfe.client_lines (
                                  client INTEGER REFERENCES pfe.clients(client_id),
                                  item INTEGER REFERENCES pfe.items(item_id),
                                  quantity INTEGER NOT NULL CHECK (quantity >= 0),
                                  PRIMARY KEY (client, item)
);

-- Tour Days
CREATE TABLE pfe.tour_days (
                               tour INTEGER REFERENCES pfe.tours(tour_id),
                               delivery_person INTEGER REFERENCES pfe.users(user_id),
                               date DATE NOT NULL,
                               PRIMARY KEY (tour,date)
);

-- Orders
CREATE TYPE pfe.order_status AS ENUM ('recupere', 'lavage', 'attente livraison', 'en cours de livraison', 'livre');

CREATE TABLE pfe.orders (
                            order_id SERIAL PRIMARY KEY,
                            client INTEGER REFERENCES pfe.clients(client_id) NOT NULL,
                            status pfe.order_status NOT NULL,
                            tour INTEGER NOT NULL,
                            date DATE NOT NULL,
                            FOREIGN KEY (tour, date) REFERENCES pfe.tour_days(tour, date)
);

-- Boxes
CREATE TYPE pfe.box_status_type AS ENUM ('livre', 'non-livre');

CREATE TABLE pfe.boxes (
                           order_id INTEGER REFERENCES pfe.orders(order_id) NOT NULL,
                           item INTEGER REFERENCES pfe.items(item_id) NOT NULL,
                           quantity INTEGER NOT NULL CHECK (quantity >= 0) DEFAULT 0,
                           delivered_qty INTEGER NOT NULL DEFAULT 0 CHECK (quantity >= 0),
                           box_status pfe.box_status_type NOT NULL,
                           PRIMARY KEY (order_id, item)
);

-- Insert data into 'users'
INSERT INTO pfe.users (first_name, last_name, email, phone, password, is_admin, is_delivery_person, is_verified)
VALUES ('Alice', 'Durand', 'alice.durand@example.com', '1234567890',
        '$2a$10$GC6lpZkl/MPs.NL6gSSgKeQjk8mK9nyot/4ZGa5EiTuc5JdpXgUXS', false, false, true),
       ('Bob', 'Lefebvre', 'bob.lefebvre@example.com', '2345678901',
        '$2a$10$GC6lpZkl/MPs.NL6gSSgKeQjk8mK9nyot/4ZGa5EiTuc5JdpXgUXS', true, true, true),
       ('Claire', 'Martin', 'claire.martin@example.com', '3456789012',
        '$2a$10$GC6lpZkl/MPs.NL6gSSgKeQjk8mK9nyot/4ZGa5EiTuc5JdpXgUXS', false, true, false),
       ('admin', 'admin', 'admin', '11111111111111111', '$2a$10$xRPu0IXEuUpep346ho8i7OOCSEz7RCyC/19WPS0DhiNxr1kl2gSv6',
        true, false, true),
       ('livreur', 'livreur', 'livreur', '08222222', '$2a$10$uy9p2gvW2QVZhlk3y9KOZO8R9Sa34O4yip42NHb85EQBRs8y2mgnq',
        false, true, true);

-- Insert data into 'tours'
INSERT INTO pfe.tours (geo_zone, delivery_person) VALUES
                                                      ('Paris', 2),
                                                      ('Lyon', 1),
                                                      ('Marseille', 2);

-- Insert data into 'clients'
INSERT INTO pfe.clients (name, address, tour) VALUES
                                                  ('Jean Dupont', '123 rue de la Paix, Paris', 1),
                                                  ('Marie Curie', '456 avenue des Champs-Élysées, Paris', 1),
                                                  ('Henri Poincaré', '789 boulevard Saint-Michel, Marseille', NULL),
                                                  ('Pierre de Fermat', '1011 rue de la République, Lyon', 2),
                                                  ('Blaise Pascal', '1213 avenue Jean Jaurès, Lyon', 2),
                                                  ('Henri le pied ron ', 'boulevard 323 la clinique, Marseille', 3);


-- Insert data into 'items'
INSERT INTO pfe.items (label, size) VALUES
                                        ('langes', 'S'),
                                        ('langes', 'M'),
                                        ('langes', 'L'),
                                        ('langes', 'XL'),
                                        ('inserts', NULL),
                                        ('sac poubelles', NULL);

-- Insert data into 'client_lines'
INSERT INTO pfe.client_lines (client, item, quantity) VALUES
                                             (1, 1, 2),
(1, 2, 1),
(1, 3, 2),
(1, 4, 1),
(1, 5, 2),
(1, 6, 1),
(2, 1, 3),
(2, 2, 1),
(2, 3, 3),
(2, 4, 1),
(2, 5, 3),
(2, 6, 1),
(3, 1, 1),
(3, 2, 1),
(3, 3, 1),
(3, 4, 1),
(3, 5, 1),
(3, 6, 1),
(4, 1, 1),
(4, 2, 1),
(4, 3, 1),
(4, 4, 1),
(4, 5, 1),
(4, 6, 1),
(5, 1, 1),
(5, 2, 1),
(5, 3, 1),
(5, 4, 1),
(5, 5, 1),
(5, 6, 1);

-- Insert data into 'tour_days'
INSERT INTO pfe.tour_days (tour, delivery_person, date) VALUES
                                                            (1, 2, '2023-12-09'),
                                                            (1, NULL, '2023-12-10'),
                                                            (2, NULL, '2023-12-10'),
                                                            (3, 1, '2023-12-10'),
                                                            (1, NULL, '2023-12-11'),
                                                            (2, NULL, '2023-12-11'),
                                                            (3, 1, '2023-12-11'),
                                                            (2, 2, '2023-12-09'),
                                                            (2, 1, '2023-12-02'),
                                                            (3, NULL, '2023-12-03');

-- Insert data into 'orders'
INSERT INTO pfe.orders (client, status, tour, date) VALUES
                                                        (1, 'recupere', 1, '2023-12-09'),
                                                        (2, 'lavage', 1, '2023-12-10'),
                                                        (1, 'attente livraison', 2, '2023-12-10'),
                                                        (3, 'attente livraison', 2, '2023-12-09'),
                                                        (1, 'en cours de livraison', 2, '2023-12-02'),
                                                        (2, 'livre', 3, '2023-12-03');

-- Insert data into 'boxes'
INSERT INTO pfe.boxes (order_id, item, quantity, delivered_qty, box_status) VALUES
                                                                                (1, 1, 2, 0, 'non-livre'),
                                                                                (1, 2, 1, 0, 'non-livre'),
                                                                                (2, 3, 3, 1, 'livre'),
                                                                                (3, 4, 1, 0, 'non-livre'),
                                                                                (4, 5, 1, 1, 'livre');





CREATE OR REPLACE PROCEDURE create_tour_day(IN input_date DATE)
LANGUAGE plpgsql
AS $$
BEGIN
    -- Insert data into 'pfe.tour_days'
INSERT INTO pfe.tour_days (tour, delivery_person, date)
SELECT DISTINCT c.tour, NULL::INTEGER, input_date
FROM pfe.client_lines cl
         JOIN pfe.clients c ON cl.client = c.client_id
         JOIN pfe.tours t ON c.tour = t.tour_id
WHERE c.tour IS NOT NULL;

-- Insert data into 'pfe.orders'
INSERT INTO pfe.orders (client, status, tour, date)
SELECT DISTINCT cl.client, 'attente livraison'::pfe.order_status, c.tour, input_date
FROM pfe.client_lines cl
         JOIN pfe.clients c ON cl.client = c.client_id
WHERE c.tour IS NOT NULL;

-- Insert data into 'pfe.boxes'
INSERT INTO pfe.boxes (order_id, item, quantity, delivered_qty, box_status)
SELECT o.order_id, cl.item, cl.quantity, 0, 'non-livre'
FROM pfe.client_lines cl
         JOIN pfe.orders o ON cl.client = o.client AND o.date = input_date
         JOIN pfe.clients c ON cl.client = c.client_id
WHERE c.tour IS NOT NULL;
END;
$$;
call create_tour_day('2023-12-12');
call create_tour_day('2023-12-13');
call create_tour_day('2023-12-14');
call create_tour_day('2023-12-15');
call create_tour_day('2023-12-16');
