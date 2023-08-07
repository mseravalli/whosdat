FROM rust AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:12-slim
COPY --from=build /app/target/release/whosdat /opt/ 
RUN mkdir -p /opt/templates
RUN mkdir -p /opt/static/pics
COPY templates/* /opt/templates
EXPOSE 8080
WORKDIR /opt
ENTRYPOINT ["/opt/whosdat"]
