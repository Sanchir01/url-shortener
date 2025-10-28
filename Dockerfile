FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /app

# Install additional dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Копируем файлы зависимостей для кэширования слоев
COPY Cargo.toml Cargo.lock ./

# Создаем пустую структуру для кэширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

# Копируем весь исходный код
COPY src ./src

# Собираем приложение с настоящим кодом
RUN touch src/main.rs  # Обновляем timestamp для пересборки
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.20 AS runner

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Копируем исполняемый файл
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/url-shortener ./url-shortener

# Копируем конфигурационный файл
COPY --from=builder /app/src/config/prod/config.toml ./config.toml

RUN chmod +x ./url-shortener

EXPOSE 4200

ENTRYPOINT ["./url-shortener"]