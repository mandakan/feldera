VERSION --try 0.8
IMPORT github.com/earthly/lib/rust:1a4a008e271c7a5583e7bd405da8fd3624c05610 AS rust
FROM ubuntu:22.04

RUN apt-get update && apt-get install --yes sudo

WORKDIR /dbsp
ENV RUSTUP_HOME=$HOME/.rustup
ENV CARGO_HOME=$HOME/.cargo
# Adds python and rust binaries to the path
ENV PATH=$HOME/.cargo/bin:$HOME/.local/bin:$PATH
ENV RUST_VERSION=1.78.0
ENV RUST_BUILD_MODE='' # set to --release for release builds
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

install-deps:
    RUN apt-get update
    RUN apt-get install --yes build-essential curl libssl-dev build-essential pkg-config \
                              cmake git gcc clang libclang-dev python3-pip python3-plumbum \
                              hub numactl openjdk-19-jre-headless maven netcat jq \
                              docker.io libenchant-2-2 graphviz locales protobuf-compiler
    # Set UTF-8 locale. Needed for the Rust compiler to handle Unicode column names.
    RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
        locale-gen
    ENV LC_ALL en_US.UTF-8
    ENV LANG en_US.UTF-8
    # NodeJS install
    RUN sudo apt-get install -y ca-certificates curl gnupg
    RUN sudo mkdir -p /etc/apt/keyrings
    RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
    ENV NODE_MAJOR=20
    RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | sudo tee /etc/apt/sources.list.d/nodesource.list
    RUN sudo apt-get update
    RUN sudo apt-get install nodejs -y
    RUN apt install unzip -y
    ## Install Bun.js
    RUN curl -fsSL https://bun.sh/install | bash
    ENV PATH="$HOME/.bun/bin:$PATH"

    RUN apt install python3-requests -y
    RUN arch=`dpkg --print-architecture`; \
            curl -LO https://github.com/redpanda-data/redpanda/releases/latest/download/rpk-linux-$arch.zip \
            && unzip rpk-linux-$arch.zip -d /bin/ \
            && rpk version \
            && rm rpk-linux-$arch.zip

    # Install Google Cloud CLI.
    RUN curl -LO https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-cli-linux-x86_64.tar.gz \
        && tar -zxf google-cloud-cli-linux-x86_64.tar.gz \
        && rm google-cloud-cli-linux-x86_64.tar.gz \
        && google-cloud-sdk/install.sh --quiet \
        && google-cloud-sdk/bin/gcloud components install beta pubsub-emulator --quiet \
        && google-cloud-sdk/bin/gcloud components list

install-rust:
    FROM +install-deps
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
        -y \
        --default-toolchain $RUST_VERSION \
        --profile minimal \
        --component clippy \
        --component rustfmt \
        --component llvm-tools-preview
    RUN rustup toolchain install nightly \
        --component clippy \
        --component rustfmt \
        --component llvm-tools-preview
    RUN cargo install --locked --force --version 0.5.0 cargo-machete
    RUN cargo install --locked --force --version 0.36.11 cargo-make
    RUN cargo install --locked --force --version 0.5.22 cargo-llvm-cov
    RUN cargo install --locked --force --version 0.1.61 cargo-chef
    RUN rustup --version
    RUN cargo --version
    RUN rustc --version
    # Switch back to this once https://github.com/earthly/earthly/issues/3718 is resolved:
    # DO rust+INIT --keep_fingerprints=true
    # Do this until then:
    ARG EARTHLY_TARGET_PROJECT_NO_TAG
    ARG EARTHLY_GIT_BRANCH
    ARG OS_RELEASE=$(md5sum /etc/os-release | cut -d ' ' -f 1)
    DO rust+INIT --cache_prefix="${EARTHLY_TARGET_PROJECT_NO_TAG}#${EARTHLY_GIT_BRANCH}#${OS_RELEASE}#earthly-cargo-cache" --keep_fingerprints=true

rust-sources:
    FROM +install-rust
    COPY --keep-ts Cargo.toml Cargo.toml
    COPY --keep-ts Cargo.lock Cargo.lock
    COPY --keep-ts openapi.json openapi.json
    COPY --keep-ts --dir crates crates
    COPY --keep-ts sql-to-dbsp-compiler/lib sql-to-dbsp-compiler/lib
    COPY --keep-ts README.md README.md

formatting-check:
    FROM +rust-sources
    DO rust+CARGO --args="+nightly fmt --all -- --check"

machete:
    FROM +rust-sources
    DO rust+CARGO --args="machete crates/"

clippy:
    FROM +rust-sources
    ENV WEBCONSOLE_BUILD_DIR=/dbsp/web-console/build
    COPY ( +build-webui/build ) ./web-console/build
    DO rust+CARGO --args="clippy -- -D warnings"
    ENV RUSTDOCFLAGS="-D warnings"
    DO rust+CARGO --args="doc --no-deps"

install-python-deps:
    FROM +install-deps
    RUN pip install wheel
    COPY --dir python ./
    RUN pip install --upgrade pip
    RUN pip wheel -r python/tests/requirements.txt --wheel-dir=wheels
    RUN pip wheel python/ --wheel-dir=wheels
    SAVE ARTIFACT wheels /wheels

install-python:
    FROM +install-deps
    COPY +install-python-deps/wheels wheels
    COPY --dir python ./
    RUN pip install --upgrade pip # remove after upgrading to ubuntu 24
    RUN pip install --user -v --no-index --find-links=wheels -r python/tests/requirements.txt
    RUN pip install --user -v --no-index --find-links=wheels feldera
    SAVE ARTIFACT /root/.local/lib/python3.10
    SAVE ARTIFACT /root/.local/bin

build-webui-deps:
    FROM +install-deps

    COPY web-console/package.json ./web-console/
    COPY web-console/bun.lockb ./web-console/
    RUN cd web-console && bun run install-frozen

build-webui:
    FROM +build-webui-deps

    COPY --dir web-console/static web-console/static
    COPY --dir web-console/src web-console/src
    COPY web-console/.prettierignore web-console/
    COPY web-console/.prettierrc web-console/
    COPY web-console/eslint.config.js web-console/
    COPY web-console/postcss.config.js web-console/
    COPY web-console/svelte.config.js ./web-console/
    COPY web-console/tailwind.config.ts ./web-console/
    COPY web-console/tsconfig.json ./web-console/
    COPY web-console/vite.config.ts ./web-console/

    # RUN cd web-console && bun run check
    RUN cd web-console && bun run build
    SAVE ARTIFACT ./web-console/build

build-dbsp:
    FROM +rust-sources
    DO rust+CARGO --args="build --package dbsp --benches"
    DO rust+CARGO --args="build --package feldera-types"
    DO rust+CARGO --args="build --package dbsp_nexmark --benches"
    DO rust+CARGO --args="build --package fda"

build-sql:
    FROM +build-dbsp
    COPY --keep-ts sql-to-dbsp-compiler sql-to-dbsp-compiler
    COPY demo/hello-world/combiner.sql demo/hello-world/combiner.sql
    COPY demo/project_demo01-TimeSeriesEnrich/project.sql demo/project_demo01-TimeSeriesEnrich/project.sql
    COPY demo/project_demo03-GreenTrip/project.sql demo/project_demo03-GreenTrip/project.sql
    COPY demo/project_demo04-SimpleSelect/project.sql demo/project_demo04-SimpleSelect/project.sql
    CACHE /root/.m2
    RUN cd "sql-to-dbsp-compiler" && ./build.sh
    SAVE ARTIFACT sql-to-dbsp-compiler/SQL-compiler/target/sql2dbsp-jar-with-dependencies.jar sql2dbsp-jar-with-dependencies.jar
    SAVE ARTIFACT sql-to-dbsp-compiler

build-adapters:
    # Adapter integration tests use the SQL compiler.
    FROM +build-sql
    DO rust+CARGO --args="build --package dbsp_adapters"

build-manager:
    FROM +build-adapters
    ENV WEBCONSOLE_BUILD_DIR=/dbsp/web-console/build
    COPY ( +build-webui/build ) ./web-console/build
    DO rust+CARGO --args="build --package pipeline-manager --features pg-embed" --output="debug/pipeline-manager"

    IF [ -f ./target/debug/pipeline-manager ]
        SAVE ARTIFACT --keep-ts ./target/debug/pipeline-manager pipeline-manager
    END
    IF [ -f ./target/release/pipeline-manager ]
        SAVE ARTIFACT --keep-ts ./target/release/pipeline-manager pipeline-manager
    END

test-sql:
    # SQL-generated code imports adapters crate.
    FROM +build-adapters
    RUN cd "sql-to-dbsp-compiler" && ./build.sh && mvn test --no-transfer-progress -q -B

build-nexmark:
    FROM +build-dbsp
    DO rust+CARGO --args="build --package dbsp_nexmark"

test-dbsp:
    FROM +build-dbsp
    ENV RUST_BACKTRACE 1
    DO rust+CARGO --args="test --package dbsp"

test-nexmark:
    FROM +build-nexmark
    ENV RUST_BACKTRACE 1
    DO rust+CARGO --args="test  --package dbsp_nexmark"
    # Perform a smoke test for nexmark with and without storage
    DO rust+CARGO --args="bench --bench nexmark -- --max-events=1000000 --cpu-cores 8 --num-event-generators 8"
    DO rust+CARGO --args="bench --bench nexmark -- --max-events=1000000 --cpu-cores 8 --num-event-generators 8 --storage 10000"

test-adapters:
    FROM +build-adapters
    DO rust+SET_CACHE_MOUNTS_ENV
    ARG DELTA_TABLE_TEST_AWS_ACCESS_KEY_ID
    ARG DELTA_TABLE_TEST_AWS_SECRET_ACCESS_KEY
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3
        RUN --mount=$EARTHLY_RUST_CARGO_HOME_CACHE --mount=$EARTHLY_RUST_TARGET_CACHE docker run -p 9092:9092 --rm -itd docker.redpanda.com/vectorized/redpanda:v23.2.3 \
            redpanda start --smp 2  && \
            (google-cloud-sdk/bin/gcloud beta emulators pubsub start --project=feldera-test --host-port=127.0.0.1:8685 &) && \
            sleep 5 && \
            RUST_BACKTRACE=1 cargo test --package dbsp_adapters --features "pubsub-emulator-test" --package sqllib
    END

test-manager:
    FROM +build-manager
    ENV PGHOST=localhost
    ENV PGUSER=postgres
    ENV PGCLIENTENCODING=UTF8
    ENV PGPORT=5432
    ENV RUST_LOG=error
    DO rust+SET_CACHE_MOUNTS_ENV
    WITH DOCKER --pull postgres
        # We just put the PGDATA in /dev/shm because the docker fs seems very slow (test time goes to 2min vs. shm 40s)
        RUN --mount=$EARTHLY_RUST_CARGO_HOME_CACHE --mount=$EARTHLY_RUST_TARGET_CACHE docker run --shm-size=512MB -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust -e PGDATA=/dev/shm -d postgres && \
            # Sleep until postgres is up (otherwise we get connection reset if we connect too early)
            # (See: https://github.com/docker-library/docs/blob/master/postgres/README.md#caveats)
            sleep 3 && \
            RUST_BACKTRACE=1 cargo test --package pipeline-manager
    END
    # We keep the test binary around so we can run integration tests later. This incantation is used to find the
    # test binary path, adapted from: https://github.com/rust-lang/cargo/issues/3670
    RUN --mount=$EARTHLY_RUST_CARGO_HOME_CACHE --mount=$EARTHLY_RUST_TARGET_CACHE cp `cargo test --features integration-test --no-run --package pipeline-manager --message-format=json | jq -r 'select(.target.kind[0] == "lib") | .executable' | grep -v null` test_binary
    SAVE ARTIFACT test_binary

openapi-checker:
    FROM +build-manager

    # Copy over pipeline manager executable
    COPY +build-manager/pipeline-manager .

    # Copy over OpenAPI spec currently in the repository
    COPY openapi.json openapi.json-base

    # Below will fail if the OpenAPI spec which is freshly generated
    # differs from the one currently in the repository
    RUN mkdir checker
    RUN cd checker && ../pipeline-manager --dump-openapi &&  \
        diff -bur openapi.json ../openapi.json-base

test-python:
    FROM +build-manager
    COPY +build-manager/pipeline-manager .
    RUN mkdir -p /root/.local/lib/python3.10
    RUN mkdir -p /root/.local/bin

    COPY +install-python/python3.10 /root/.local/lib/python3.10
    COPY +install-python/bin /root/.local/bin

    COPY +build-manager/pipeline-manager .
    COPY +build-sql/sql-to-dbsp-compiler sql-to-dbsp-compiler

    COPY python/tests tests

    # Reuse `Cargo.lock` to ensure consistent crate versions.
    RUN mkdir -p /working-dir/cargo_workspace
    COPY Cargo.lock /working-dir/cargo_workspace/Cargo.lock

    ENV PGHOST=localhost
    ENV PGUSER=postgres
    ENV PGCLIENTENCODING=UTF8
    ENV PGPORT=5432
    ENV RUST_LOG=error
    ENV WITH_POSTGRES=1
    ENV IN_CI=1
    ENV KAFKA_URL="localhost:9092"
    WITH DOCKER --pull postgres
        RUN docker run --shm-size=512MB -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust -e PGDATA=/dev/shm -d postgres && \
            sleep 10 && \
            (./pipeline-manager --bind-address=0.0.0.0 --api-server-working-directory=/working-dir --compiler-working-directory=/working-dir --runner-working-directory=/working-dir --sql-compiler-home=/dbsp/sql-to-dbsp-compiler --dbsp-override-path=/dbsp --db-connection-string=postgresql://postgres:postgres@localhost:5432 --compilation-profile=unoptimized &) && \
            sleep 5 && \
            cd tests && python3 -m pytest .
    END

test-rust:
    BUILD +test-dbsp
    BUILD +test-nexmark
    BUILD +test-adapters
    BUILD +test-manager

# TODO: the following two container tasks duplicate work that we otherwise do in the Dockerfile,
# but by mostly repeating ourselves, we can reuse earlier Earthly stages to speed up the CI.
build-pipeline-manager-container:
    FROM +install-deps
    RUN useradd -ms /bin/bash feldera
    USER feldera
    WORKDIR /home/feldera

    # First, copy over the artifacts built from previous stages
    RUN mkdir -p database-stream-processor/sql-to-dbsp-compiler/SQL-compiler/target
    COPY +build-manager/pipeline-manager .
    COPY +build-sql/sql2dbsp-jar-with-dependencies.jar database-stream-processor/sql-to-dbsp-compiler/SQL-compiler/target/

    # Reuse `Cargo.lock` to ensure consistent crate versions.
    RUN mkdir -p .feldera/cargo_workspace
    COPY --chown=feldera Cargo.lock .feldera/cargo_workspace/Cargo.lock

    # Then copy over the crates needed by the sql compiler
    COPY crates/dbsp database-stream-processor/crates/dbsp
    COPY crates/feldera-types database-stream-processor/crates/feldera-types
    COPY crates/adapters database-stream-processor/crates/adapters
    COPY crates/nexmark database-stream-processor/crates/nexmark
    COPY README.md database-stream-processor/README.md

    # Then copy over the required SQL compiler files
    COPY sql-to-dbsp-compiler/SQL-compiler/sql-to-dbsp database-stream-processor/sql-to-dbsp-compiler/SQL-compiler/sql-to-dbsp
    COPY sql-to-dbsp-compiler/lib database-stream-processor/sql-to-dbsp-compiler/lib
    COPY sql-to-dbsp-compiler/temp database-stream-processor/sql-to-dbsp-compiler/temp
    ENV RUSTUP_HOME=$HOME/.rustup
    ENV CARGO_HOME=$HOME/.cargo

    # Install cargo and rust for this non-root user
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    ENV PATH="$PATH:/home/feldera/.cargo/bin"

    RUN ./pipeline-manager --bind-address=0.0.0.0 --sql-compiler-home=/home/feldera/database-stream-processor/sql-to-dbsp-compiler --compilation-profile=unoptimized --dbsp-override-path=/home/feldera/database-stream-processor --precompile
    ENTRYPOINT ["./pipeline-manager", "--bind-address=0.0.0.0", "--sql-compiler-home=/home/feldera/database-stream-processor/sql-to-dbsp-compiler", "--dbsp-override-path=/home/feldera/database-stream-processor", "--compilation-profile=unoptimized"]

# Same as the above, but with a permissive CORS setting, else playwright doesn't work
pipeline-manager-container-cors-all:
    FROM +build-pipeline-manager-container
    ENTRYPOINT ["./pipeline-manager", "--bind-address=0.0.0.0", "--sql-compiler-home=/home/feldera/database-stream-processor/sql-to-dbsp-compiler", "--dbsp-override-path=/home/feldera/database-stream-processor", "--dev-mode", "--compilation-profile=unoptimized"]

# TODO: mirrors the Dockerfile. See note above.
build-demo-container:
    FROM +install-rust
    WORKDIR /
    # Install snowsql
    RUN curl -O https://sfc-repo.snowflakecomputing.com/snowsql/bootstrap/1.2/linux_x86_64/snowsql-1.2.28-linux_x86_64.bash \
        && SNOWSQL_DEST=/bin SNOWSQL_LOGIN_SHELL=~/.profile bash snowsql-1.2.28-linux_x86_64.bash \
        && snowsql -v
    COPY +install-python/python3.10 /root/.local/lib/python3.10
    COPY +install-python/bin /root/.local/bin
    # Needed by the JDBC demo.
    RUN pip3 install "psycopg[binary]"
    # Needed by the simple-count demo.
    RUN pip3 install kafka-python
    COPY demo demo
    CMD bash

build-kafka-connect-container:
    FROM DOCKERFILE -f deploy/Dockerfile --target kafka-connect .

test-docker-compose:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container
        RUN COMPOSE_HTTP_TIMEOUT=120 SECOPS_DEMO_ARGS="--prepare-args 200000" RUST_LOG=debug,tokio_postgres=info docker-compose -f docker-compose.yml --profile demo up --force-recreate --exit-code-from demo
    END

# Test whether the stable container image runs with our Docker compose file
# and whether we can migrate from the last stable version to the latest version
test-docker-compose-stable:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    ENV FELDERA_VERSION=0.25.0
    RUN apk --no-cache add curl
    WITH DOCKER --pull postgres \
                --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --pull ghcr.io/feldera/pipeline-manager:0.25.0 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --pull ghcr.io/feldera/demo-container:0.25.0
        RUN COMPOSE_HTTP_TIMEOUT=120 SECOPS_DEMO_ARGS="--prepare-args 200000" RUST_LOG=debug,tokio_postgres=info docker-compose -f docker-compose.yml --profile demo up --force-recreate --exit-code-from demo && \
            # This should run the latest version of the code and in the process, trigger a migration.
            COMPOSE_HTTP_TIMEOUT=120 SECOPS_DEMO_ARGS="--prepare-args 200000" FELDERA_VERSION=latest RUST_LOG=debug,tokio_postgres=info docker-compose -f docker-compose.yml up -d db pipeline-manager redpanda && \
            sleep 10 && \
            # Exercise a few simple workflows in the API
            curl http://localhost:8080/v0/programs &&  \
            curl http://localhost:8080/v0/pipelines &&  \
            curl http://localhost:8080/v0/connectors
    END

test-debezium-mysql:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/docker-compose-debezium-mysql.yml .
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --pull debezium/example-mysql:2.5 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container \
                --load ghcr.io/feldera/kafka-connect:latest=+build-kafka-connect-container
        RUN COMPOSE_HTTP_TIMEOUT=120 RUST_LOG=debug,tokio_postgres=info docker-compose -f docker-compose.yml -f docker-compose-debezium-mysql.yml --profile debezium up --force-recreate --exit-code-from debezium-demo
    END

test-debezium-postgres:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/docker-compose-debezium-postgres.yml .
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --pull debezium/example-postgres:2.5 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container \
                --load ghcr.io/feldera/kafka-connect:latest=+build-kafka-connect-container
        RUN COMPOSE_HTTP_TIMEOUT=120 RUST_LOG=debug,tokio_postgres=info docker-compose -f docker-compose.yml -f docker-compose-debezium-postgres.yml --profile debezium up --force-recreate --exit-code-from debezium-demo
    END

test-debezium-jdbc-sink:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/docker-compose-jdbc.yml .
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --pull debezium/example-postgres:2.3 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container \
                --load ghcr.io/feldera/kafka-connect:latest=+build-kafka-connect-container
        RUN COMPOSE_HTTP_TIMEOUT=120 RUST_LOG=debug,tokio_postgres=info docker-compose -f docker-compose.yml -f docker-compose-jdbc.yml --profile debezium up --force-recreate --exit-code-from debezium-jdbc-demo
    END

test-snowflake:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/.env .
    RUN cat .env
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container \
                --load ghcr.io/feldera/kafka-connect:latest=+build-kafka-connect-container
        RUN COMPOSE_HTTP_TIMEOUT=120 RUST_LOG=debug,tokio_postgres=info docker-compose --env-file .env -f docker-compose.yml --profile snowflake up --force-recreate --exit-code-from snowflake-demo
    END

test-s3:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/.env .
    RUN cat .env
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container \
                --load ghcr.io/feldera/kafka-connect:latest=+build-kafka-connect-container
        RUN COMPOSE_HTTP_TIMEOUT=120 RUST_LOG=debug,tokio_postgres=info docker-compose --env-file .env -f docker-compose.yml --profile s3 up --force-recreate --exit-code-from s3-demo
    END

test-service-related:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/.env .
    RUN cat .env
    ENV FELDERA_VERSION=latest
    WITH DOCKER --pull docker.redpanda.com/vectorized/redpanda:v23.2.3 \
                --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --load ghcr.io/feldera/demo-container:latest=+build-demo-container
        RUN COMPOSE_HTTP_TIMEOUT=120 RUST_LOG=debug,tokio_postgres=info docker-compose --env-file .env -f docker-compose.yml --profile demo-service-related up --force-recreate --exit-code-from demo-service-related
    END

# Fetches the test binary from test-manager, and produces a container image out of it
integration-test-container:
    FROM +install-deps
    COPY +test-manager/test_binary .
    # Check that the binary does indeed run integration tests
    RUN ./test_binary integration_test --list | grep integration_test
    ENV TEST_DBSP_URL=http://pipeline-manager:8080
    ENV RUST_BACKTRACE=1
    ENTRYPOINT ["./test_binary", "integration_test::"]
    SAVE IMAGE itest:latest

# Runs the integration test container against the docker compose setup
integration-tests:
    FROM earthly/dind:alpine
    COPY deploy/docker-compose.yml .
    COPY deploy/.env .
    ENV FELDERA_VERSION=latest
    WITH DOCKER --load ghcr.io/feldera/pipeline-manager:latest=+build-pipeline-manager-container \
                --compose docker-compose.yml \
                --service pipeline-manager \
                --load itest:latest=+integration-test-container
        # Output pipeline manager logs if tests fail. Without this we don't have
        # a way to see what went wrong in the manager.
        RUN sleep 15 && docker run --env-file .env --network default_default itest:latest; \
            status=$?; \
            [ $status -ne 0 ] && docker logs default-pipeline-manager-1; \
            exit $status
    END

ui-playwright-container:
    FROM +install-deps
    COPY web-console web-console
    # COPY web-console/playwright-ct.config.ts web-console
    COPY deploy/docker-compose.yml .
    COPY deploy/.env .

    # Pull playwright-snapshots for visual regression testing
    # It was decided it's better to clone the snapshots repo during the build rather than have it as a submodule
    ARG PLAYWRIGHT_SNAPSHOTS_COMMIT
    RUN echo PLAYWRIGHT_SNAPSHOTS_COMMIT=$PLAYWRIGHT_SNAPSHOTS_COMMIT
    GIT CLONE --branch=$PLAYWRIGHT_SNAPSHOTS_COMMIT https://github.com/feldera/playwright-snapshots.git playwright-snapshots

    WORKDIR web-console
    RUN bun run install-frozen
    # Generate .svelte-kit/tsconfig.json and .svelte-kit/types
    RUN bunx svelte-kit sync
    RUN bunx playwright install
    RUN bunx playwright install-deps
    ENV CI=true
    ENV PLAYWRIGHT_API_ORIGIN=http://localhost:8080/
    ENV PLAYWRIGHT_APP_ORIGIN=http://localhost:8080/
    ENV DISPLAY=

    # Install docker compose - earthly can do this automatically, but it installs an older version
    ENV DOCKER_CONFIG=${DOCKER_CONFIG:-$HOME/.docker}
    RUN mkdir -p $DOCKER_CONFIG/cli-plugins
    RUN curl -SL https://github.com/docker/compose/releases/download/v2.24.0-birthday.10/docker-compose-linux-x86_64 -o $DOCKER_CONFIG/cli-plugins/docker-compose
    RUN chmod +x $DOCKER_CONFIG/cli-plugins/docker-compose

    # Install zip to prepare test artifacts for export
    RUN apt-get install -y zip

# ui-playwright-tests-e2e:
#     FROM +ui-playwright-container
#     ENV FELDERA_VERSION=latest

#     TRY
#         WITH DOCKER --load ghcr.io/feldera/pipeline-manager:latest=+pipeline-manager-container-cors-all \
#                     --compose ../docker-compose.yml \
#                     --service pipeline-manager
#             # We zip artifacts regardless of test success or error, and then we complete the command preserving test's exit_code
#             RUN sleep 10 && if bunx playwright test -c playwright-e2e.config.ts; then exit_code=0; else exit_code=$?; fi \
#                 && cd /dbsp \
#                 && zip -r playwright-report-e2e.zip playwright-report-e2e \
#                 && zip -r test-results-e2e.zip test-results-e2e \
#                 && exit $exit_code
#         END
#     FINALLY
#         SAVE ARTIFACT --if-exists /dbsp/playwright-report-e2e.zip AS LOCAL ./playwright-artifacts/
#         SAVE ARTIFACT --if-exists /dbsp/test-results-e2e.zip      AS LOCAL ./playwright-artifacts/
#     END

ui-playwright-tests-ct:
    FROM +ui-playwright-container
    ENV FELDERA_VERSION=latest

    TRY
        # We zip artifacts regardless of test success or error, and then we complete the command preserving test's exit_code
        RUN if bunx playwright test -c playwright-ct.config.ts; then exit_code=0; else exit_code=$?; fi \
        && cd /dbsp/web-console \
        && zip -r playwright-report-ct.zip playwright-report-ct \
        && zip -r test-results-ct.zip test-results-ct \
        && exit $exit_code
    FINALLY
        SAVE ARTIFACT --if-exists /dbsp/web-console/playwright-report-ct.zip AS LOCAL ./playwright-artifacts/
        SAVE ARTIFACT --if-exists /dbsp/web-console/test-results-ct.zip      AS LOCAL ./playwright-artifacts/
    END

benchmark:
    FROM +build-manager
    COPY demo/project_demo12-HopsworksTikTokRecSys/tiktok-gen demo/project_demo12-HopsworksTikTokRecSys/tiktok-gen
    COPY scripts/bench.bash scripts/bench.bash
    COPY benchmark/feldera-sql/run.py benchmark/feldera-sql/run.py
    COPY benchmark/feldera-sql/benchmarks benchmark/feldera-sql/benchmarks
    COPY +build-manager/pipeline-manager .
    COPY +build-sql/sql-to-dbsp-compiler sql-to-dbsp-compiler
    RUN mkdir -p /working-dir/cargo_workspace
    COPY Cargo.lock /working-dir/cargo_workspace/Cargo.lock
    ENV PGHOST=localhost
    ENV PGUSER=postgres
    ENV PGCLIENTENCODING=UTF8
    ENV PGPORT=5432
    ENV RUST_LOG=error
    ENV WITH_POSTGRES=1
    ENV IN_CI=1
    WITH DOCKER --pull postgres
        RUN docker run --shm-size=512MB -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust -e PGDATA=/dev/shm -d postgres && \
            sleep 10 && \
            (./pipeline-manager --bind-address=0.0.0.0 --api-server-working-directory=/working-dir --compiler-working-directory=/working-dir --runner-working-directory=/working-dir --sql-compiler-home=/dbsp/sql-to-dbsp-compiler --dbsp-override-path=/dbsp --db-connection-string=postgresql://postgres:postgres@localhost:5432 --compilation-profile=optimized &) && \
            sleep 5 && \
            docker run --name redpanda -p 9092:9092 --rm -itd docker.redpanda.com/vectorized/redpanda:v23.2.3 redpanda start --smp 2 \
            && bash scripts/bench.bash
    END
    SAVE ARTIFACT benchmark-run-data AS LOCAL .

flink-benchmark:
    FROM +rust-sources

    # Install docker compose - earthly can do this automatically, but it installs an older version
    ENV DOCKER_CONFIG=${DOCKER_CONFIG:-$HOME/.docker}
    RUN mkdir -p $DOCKER_CONFIG/cli-plugins
    RUN curl -SL https://github.com/docker/compose/releases/download/v2.24.0-birthday.10/docker-compose-linux-x86_64 -o $DOCKER_CONFIG/cli-plugins/docker-compose
    RUN chmod +x $DOCKER_CONFIG/cli-plugins/docker-compose
    RUN mkdir -p benchmark/flink
    COPY benchmark/flink/* benchmark/flink
    COPY benchmark/run-nexmark.sh benchmark
    RUN apt-get -y install maven bc
    RUN benchmark/flink/setup-flink.sh

    # Run with rocksdb
    COPY benchmark/flink/flink-conf-rocksdb.yaml benchmark/flink/flink-conf.yaml
    WITH DOCKER
        RUN cd benchmark && ./run-nexmark.sh -r flink --query all --events 100M && mv nexmark.csv flink_results.csv
    END
    RUN cat benchmark/flink/flink-conf.yaml
    SAVE ARTIFACT benchmark/flink_results.csv AS LOCAL flink_results_rocksdb.csv

    # Run with hashmap
    COPY benchmark/flink/flink-conf-hashmap.yaml benchmark/flink/flink-conf.yaml
    WITH DOCKER
        RUN cd benchmark && ./run-nexmark.sh -r flink --query all --events 100M && mv nexmark.csv flink_results.csv
    END
    SAVE ARTIFACT benchmark/flink_results.csv AS LOCAL flink_results_hashmap.csv

ci-tests:
    BUILD +formatting-check
    BUILD +machete
    BUILD +clippy
    BUILD +test-rust
    BUILD +openapi-checker
    BUILD +test-sql
    BUILD +integration-tests
    BUILD +ui-playwright-tests-ct
    # BUILD +test-docker-compose-stable
    # TODO: Temporarily disabled while we port the demo script
    # BUILD +test-snowflake
    # BUILD +test-s3

nightly-tests:
    BUILD +test-python
    BUILD +test-debezium-postgres
    BUILD +test-debezium-jdbc-sink
    BUILD +test-debezium-mysql
    BUILD +test-docker-compose
    BUILD +test-service-related
    # BUILD +ui-playwright-tests-e2e
