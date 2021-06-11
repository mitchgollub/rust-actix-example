run:
    just build-app && \
    cd perfect_movie_game_backend && \
    cargo run && \
    cd - 

watch:
    git ls-files | entr -s 'kill -2 $(lsof -t -i:$(echo $PORT)); just run &'

build-app:
    cd perfect_movie_game_frontend && \
    RUST_LOG=error trunk build && \
    cd -