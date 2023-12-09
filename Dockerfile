# Utilisez une image officielle de Rust comme base
FROM rust:latest

# Créez un répertoire de travail dans l'image
WORKDIR /app

# Copiez le fichier Cargo.toml et le fichier Cargo.lock pour permettre la mise en cache des dépendances
COPY Cargo.toml .env  ./

# Copiez tout le dossier src
COPY src ./src

#RUN echo "Démarrage de la construction de l'application..."
#RUN cargo build --release

# Compilez l'application
# Si votre application utilise un port, exposez-le
EXPOSE 4125

CMD [ "cargo","run" ]

