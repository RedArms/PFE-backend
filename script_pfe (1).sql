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
    password VARCHAR(255) NOT NULL UNIQUE,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    is_delivery_person BOOLEAN NOT NULL DEFAULT false,
    is_verified BOOLEAN NOT NULL DEFAULT false
);

-- Tours
CREATE TABLE pfe.tours (
    tour_id SERIAL PRIMARY KEY,
    geo_zone VARCHAR(255) NOT NULL,
    delivery_person INTEGER REFERENCES pfe.users(user_id)
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
    quantity INTEGER NOT NULL CHECK (quantity > 0),
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
    client INTEGER REFERENCES pfe.clients(client_id),
    status pfe.order_status,
    tour INTEGER,
    date DATE,
    FOREIGN KEY (tour, date) REFERENCES pfe.tour_days(tour, date)
);

-- Boxes
CREATE TYPE pfe.box_status_type AS ENUM ('livre', 'non-livre');

CREATE TABLE pfe.boxes (
    order_id INTEGER REFERENCES pfe.orders(order_id),
    item INTEGER REFERENCES pfe.items(item_id),
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    delivered_qty INTEGER DEFAULT 0 CHECK (quantity >= 0),
    box_status pfe.box_status_type,
    PRIMARY KEY (order_id, item)
);

-- Insert data into 'users'
INSERT INTO pfe.users (first_name, last_name, email, phone, password, is_admin, is_delivery_person, is_verified) VALUES
('Alice', 'Durand', 'alice.durand@example.com', '1234567890', 'password123', false, false, true),
('Bob', 'Lefebvre', 'bob.lefebvre@example.com', '2345678901', 'password234', true, true, true),
('Claire', 'Martin', 'claire.martin@example.com', '3456789012', 'password345', false, true, false);

-- Insert data into 'tours'
INSERT INTO pfe.tours (geo_zone, delivery_person) VALUES
('Paris', 2),
('Lyon', 3),
('Marseille', NULL);

-- Insert data into 'clients'
INSERT INTO pfe.clients (name, address, tour) VALUES
('Jean Dupont', '123 rue de la Paix, Paris', 1),
('Marie Curie', '456 avenue des Champs-Élysées, Paris', 1),
('Henri Poincaré', '789 boulevard Saint-Michel, Marseille', NULL);

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
(2, 3, 3),
(2, 4, 1),
(3, 5, 1);

-- Insert data into 'tour_days'
INSERT INTO pfe.tour_days (tour, delivery_person, date) VALUES
(1, 2, '2023-12-01'),
(2, 3, '2023-12-02'),
(3, NULL, '2023-12-03');

-- Insert data into 'orders'
INSERT INTO pfe.orders (client, status, tour, date) VALUES
(1, 'recupere', 1, '2023-12-01'),
(2, 'lavage', 1, '2023-12-01'),
(3, 'attente livraison', 2, '2023-12-02'),
(1, 'en cours de livraison', 2, '2023-12-02'),
(2, 'livre', 3, '2023-12-03');

-- Insert data into 'boxes'
INSERT INTO pfe.boxes (order_id, item, quantity, delivered_qty, box_status) VALUES
(1, 1, 2, 0, 'non-livre'),
(1, 2, 1, 0, 'non-livre'),
(2, 3, 3, 1, 'livre'),
(3, 4, 1, 0, 'non-livre'),
(4, 5, 1, 1, 'livre');