<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="den-logo">
</p>
<p align="center">
    <h1 align="center">DEN</h1>
</p>
<p align="center">
    <em>Amplify events, empower notifications, elevate observability.</em>
</p>
<p align="center">
  <a href="https://crates.io/crates/den">
    <img src="https://img.shields.io/badge/den-v0.1.3-0080ff?style=flat&logo=rust" alt="version">
  </a>
  <a href="https://github.com/atareao/den/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/atareao/den?style=flat&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
  </a>
  <a href="https://github.com/atareao/den/releases">
    <img src="https://img.shields.io/github/v/release/atareao/den?style=flat&logo=github&logoColor=white&color=0080ff" alt="release">
  </a>
  <a href="https://github.com/atareao/den/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/atareao/den/ci.yml?style=flat&logo=githubactions&logoColor=white&color=0080ff" alt="build">
  </a>
  <br>
  <a href="https://hub.docker.com/r/atareao/den">
    <img src="https://img.shields.io/docker/pulls/atareao/den?style=flat&logo=docker&logoColor=white&color=0080ff" alt="docker-pulls">
  </a>
  <a href="https://github.com/atareao/den/stargazers">
    <img src="https://img.shields.io/github/stars/atareao/den?style=flat&logo=star&logoColor=white&color=0080ff" alt="stars">
  </a>
  <a href="https://github.com/atareao/den/network">
    <img src="https://img.shields.io/github/forks/atareao/den?style=flat&logo=git&logoColor=white&color=0080ff" alt="forks">
  </a>
  <a href="https://github.com/atareao/den/blob/main/Cargo.toml">
    <img src="https://img.shields.io/badge/Rust-2021-0080ff?style=flat&logo=rust&logoColor=white" alt="rust-edition">
  </a>
  <a href="https://github.com/atareao/den/blob/main/LICENSE">
    <img src="https://img.shields.io/github/languages/top/atareao/den?style=flat&logo=rust&logoColor=white&color=0080ff" alt="language">
  </a>
  <a href="https://github.com/atareao/den/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/atareao/den?style=flat&logo=people&logoColor=white&color=0080ff" alt="contributors">
  </a>
<p>

<br>

## Overview

**DEN** is a lightweight Docker Event Notification daemon written in [Rust](https://www.rust-lang.org/). It listens to the Docker event stream and forwards matching events to your preferred messaging or observability platforms.

### Supported Publishers

| Service       | Protocol     | Use Case              |
|---------------|-------------|-----------------------|
| Telegram      | HTTP Bot API| Chat notifications    |
| Mattermost    | Webhook     | Team messaging        |
| Discord       | Webhook     | Community alerts      |
| Slack         | Webhook     | Workplace alerts      |
| Matrix        | Client-Server API | Decentralized chat|
| RabbitMQ      | AMQP        | Message queue         |
| Mosquitto     | MQTT        | IoT / lightweight     |
| ZincObserve   | HTTP API    | Log aggregation       |

---

## Quick Start

### Docker Compose

```yaml
services:
  den:
    image: atareao/den:latest
    container_name: den
    init: true
    restart: unless-stopped
    hostname: co1
    environment:
      RUST_LOG: debug
    volumes:
      - ./config.yml:/app/config.yml
      - /var/run/docker.sock:/var/run/docker.sock
```

### Docker CLI

```bash
docker run -d \
  --name den \
  --init \
  --restart unless-stopped \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v $(pwd)/config.yml:/app/config.yml \
  -e RUST_LOG=debug \
  atareao/den:latest
```

### From Source

```bash
git clone https://github.com/atareao/den
cd den
cargo build --release
./target/release/den
```

---

## Container Monitoring Labels

Control which containers are monitored using Docker labels:

| `monitorize_always` | Label                        | Behavior               |
|---------------------|------------------------------|------------------------|
| `true` (default)    | *(no label)*                 | Monitored              |
| `true`              | `es.atareao.den.monitorize=false` | **Excluded**     |
| `false`             | *(no label)*                 | **Not monitored**      |
| `false`             | `es.atareao.den.monitorize=true`  | Monitored          |

```bash
# Exclude a container from monitoring
docker run --label es.atareao.den.monitorize=false --rm hello-world

# Only monitor containers with this label (when monitorize_always=false)
docker run --label es.atareao.den.monitorize=true --rm hello-world
```

---

## Configuration

Full configuration reference in [`config.sample.yml`](./config.sample.yml):

```yaml
settings:
  monitorize_always: true

objects:
  - name: container
    monitorize: true
    events:
      - name: "health_status: unhealthy"
        message: "📦🤒 Container unhealthy\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: {{hostname}}\nContainer: {{container}}\nImage: {{image}}"
      - name: destroy
        message: "📦💥 Destroyed container\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: {{hostname}}\nContainer: {{container}}\nImage: {{image}}"
      - name: stop
        message: "📦✋ Stopped container\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: **{{hostname}}**\nContainer: **{{container}}**\nImage: **{{image}}**"
      - name: start
        message: "📦🏁 Started container\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: **{{hostname}}**\nContainer: **{{container}}**\nImage: **{{image}}**"
      - name: create
        message: "### 📦🆕 Created container\n* DateTime: {{ timestamp|datetimeformat(format='iso') }}\n* Hostname: **{{hostname}}**\n* Container: **{{container}}**\n* Image: **{{image}}**"
      - name: die
        message: "📦☠️ Died container\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: {{hostname}}\nContainer: {{container}}\nImage: {{image}}"
  - name: image
    monitorize: true
    events:
      - name: delete
        message: Deleted image
  - name: volume
    monitorize: true
    events:
      - name: destroy
        message: "🥃💥 Volume destroyed\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: {{hostname}}\nVolume: {{volume}}"
      - name: create
        message: "🥃🆕 Volume created\nDateTime: {{now | date(format='%H:%M:%S %d-%m-%Y', timezone='Europe/Madrid')}}\nHostname: {{hostname}}\nVolume: {{volume}}"
  - name: network
    monitorize: true
    events:
      - name: destroy
        message: "🕸️💥 Network destroyed\nDateTime: {{ timestamp|datetimeformat(format='iso') }}\nHostname: {{hostname}}\nNetwork: {{network}}"

publishers:
  - service: slack
    enabled: false
    config:
      url: https://hooks.slack.com/services/<your_uuid>
  - service: discord
    enabled: false
    config:
      url: https://discordapp.com/api/webhooks/<your_uuid>
  - service: mattermost
    enabled: false
    config:
      url: https://mm.your-site.com
      token: xxxxxxxxxxxxxxxxxxxx
      channel_id: xxxxxxxxxxxxxxxxxxx
  - service: telegram
    enabled: false
    config:
      url: https://api.telegram.org
      token: <BOT_TOKEN>
      chat_id: <CHAT_ID>
  - service: zinc
    enabled: true
    config:
      url: https://zincobserve.your-site.com
      token: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
      index: docker
  - service: matrix
    enabled: false
    config:
      url: matrix.your-site.com
      token: xxxxxxxxxxxxxxxxxxxxxxxxxxx
      room: "!xxxxxxxxxxxxxxxxxxxxxxxx"
  - service: mosquitto
    enabled: true
    config:
      user: guest
      password: guest
      host: localhost
      port: 1883
      topic: docker/events
  - service: rabbitmq
    enabled: false
    config:
      user: guest
      password: guest
      host: localhost
      port: 5672
      queue: docker
```

---

## Message Templates

Messages use [MiniJinja](https://github.com/mitsuhiko/minijinja) templates with these variables:

| Variable      | Description                  | Available For       |
|---------------|------------------------------|---------------------|
| `{{hostname}}`| Docker host hostname         | All objects         |
| `{{timestamp}}`| Unix timestamp of the event | All objects         |
| `{{id}}`      | Event actor ID               | All objects         |
| `{{container}}`| Container name              | Container events    |
| `{{image}}`   | Container image              | Container events    |
| `{{volume}}`  | Volume name                  | Volume events       |
| `{{network}}` | Network name                 | Network events      |
| `{{type}}`    | Network type                 | Network events      |

A custom `datetimeformat` filter is available:

```
{{ timestamp|datetimeformat(format='iso') }}
{{ timestamp|datetimeformat(format='%H:%M:%S', timezone='Europe/Madrid') }}
```

---

## Repository Structure

```
den/
├── Cargo.toml          # Rust dependencies & metadata
├── Dockerfile          # Multi-stage Docker build
├── config.sample.yml   # Configuration reference
├── docker-compose.yml  # Compose example
├── entrypoint.sh       # Container entrypoint
├── run.sh              # Init helper
└── src/
    ├── main.rs         # Event loop & orchestration
    ├── config.rs       # YAML config parser
    ├── object.rs       # Docker event/object models
    ├── publisher.rs    # Publisher implementations
    ├── filters.rs      # Template filters (datetime)
    └── error.rs        # Custom error types
```

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Commit your changes
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

---

## License

[MIT](./LICENSE) &mdash; Copyright &copy; 2022 Lorenzo Carbonell