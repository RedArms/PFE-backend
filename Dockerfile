# Utilisez une image officielle de Rust comme base
FROM rust:latest

# Créez un répertoire de travail dans l'image
WORKDIR /app

# Copiez le fichier Cargo.toml et le fichier Cargo.lock pour permettre la mise en cache des dépendances
COPY Cargo.toml .env  ./

# Copiez tout le dossier src
COPY src ./src

# Compilez l'application
# Si votre application utilise un port, exposez-le
EXPOSE 8080

CMD [ "cargo","run" ]