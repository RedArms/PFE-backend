# Utilisez une image officielle de Rust comme base
FROM rust:latest

ARG PORT

# Créez un répertoire de travail dans l'image
WORKDIR /app

# Copiez le fichier Cargo.toml et le fichier Cargo.lock pour permettre la mise en cache des dépendances
COPY Cargo.toml .env  ./

# Copiez tout le dossier src
COPY src ./src

#RUN echo "Démarrage de la construction de l'application..."
#RUN cargo build --release

# for being observed 
LABEL com.centurylinklabs.watchtower.enable="true"

# Compilez l'application
# Si votre application utilise un port, exposez-le
EXPOSE ${PORT}

CMD [ "cargo","run" ]

