<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="project-logo">
</p>
<p align="center">
    <h1 align="center">DEN</h1>
</p>
<p align="center">
    <em>Amplify events, empower notifications, elevate observability.</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/atareao/den?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/atareao/den?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/atareao/den?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/atareao/den?style=default&color=0080ff" alt="repo-language-count">
<p>
<p align="center">
	<!-- default option, no dependency badges. -->
</p>

<br><!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary><br>

- [ Overview](#-overview)
- [ Features](#-features)
- [ Repository Structure](#-repository-structure)
- [ Modules](#-modules)
- [ Getting Started](#-getting-started)
  - [ Installation](#-installation)
  - [ Usage](#-usage)
  - [ Tests](#-tests)
- [ Project Roadmap](#-project-roadmap)
- [ Contributing](#-contributing)
- [ License](#-license)
- [ Acknowledgments](#-acknowledgments)
</details>
<hr>

##  Overview

A Docker Event Notification

A very simple app in a Docker to notificate docker events with some Messaging services as,

* Telegram
* Mattermost
* Discord
* ZincObeserve
* Matrix
* Rabbitmq
* Mosquitto

### Avoid container monitoring

if `monitorize_always=true` all container are monitorized except has those who has the label `es.atareao.den.monitorize=false`. But if `monitorize_always=false` only container with the label `es.atareao.den.monitorize=true` will be monitorized.

So if you set `es.atareao.den.monitorize=false` the container never will be monitorized, also if `monitorize_always=true`

```bash
docker run --label es.atareao.den.monitorize=false --rm hello-world
```

If you want, only some contaniers will be monitorized, you mast set `monitorize_always=false`, and set `es.atareao.den.monitorize=true`, in those containers you want monitorize.

By the moment, only container have this attribute. I think in next version I can implement this feature for other Docker objects.

### Configuration

```bash
settings:
  # if `monitorize_always=true` all container are monitorized except has the
  # label `es.atareao.den.monitorize=false`
  # if `monitorize_always=false` only container with the following label
  # `es.atareao.den.monitorize=true` will be monitorized
  monitorize_always: true

objects:
  #  https://docs.docker.com/engine/reference/commandline/events/
  - name: container
    monitorize: true
    # attach, commit, copy, create, destroy, detach, die, exec_create,
    # exec_detach, exec_die, exec_start, export, health_status, kill, oom,
    # pause, rename, resize, restart, start, stop, top, unpause, update
    events:
      - name: 'health_status: unhealthy'
        message: "📦🤒 Container unhealty
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: {{hostname}}\n
            Container: {{container}}\n
            Image: {{image}}"
      - name: destroy
        message: "📦💥 Destroyed container\n
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: {{hostname}}\n
            Container: {{container}}\n
            Image: {{image}}"
      - name: stop
        message: "📦✋ Stopped container
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: **{{hostname}}**\n
            Container: **{{container}}**\n
            Image: **{{image}}**"
      - name: start
        message: "📦🏁 Started container\n
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: **{{hostname}}**\n
            Container: **{{container}}**\n
            Image: **{{image}}**"
      - name: create
        message: "### 📦🆕 Created container\n
            * DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            * Hostname: **{{hostname}}**\n
            * Container: **{{container}}**\n
            * Image: **{{image}}**"
      - name: die
        message: "📦☠️  Died container\n
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: {{hostname}}\n
            Container: {{container}}\n
            Image: {{image}}"
  - name: image
    monitorize: true
    # delete, import, load, pull, push, save, tag, untag
    events:
      - name: delete
        message: Deleted image
  - name: plugin
    monitorize: false
    # enable, disable, install, remove
    events: []
  - name: volume
    monitorize: true
    # create, destroy, mount, unmount
    events:
      - name: destroy
        message: "🥃💥 Volume destroyed
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: {{hostname}}\n
            Volume: {{volume}}"
      - name: create
        message: "🥃🆕 Volume created \n
            DateTime: {{now | date(format='%H:%M:%S %d-%m-%Y', timezone='Europe/Madrid')}}\n
            Hostname: {{hostname}}\n
            Volume: {{volume}}"
  - name: network
    monitorize: true
    # create, connect, destroy, disconnect, remove
    events:
      - name: destroy
        message: "🕸️💥 Network destroyed\n
            DateTime: {{ timestamp|datetimeformat(format='iso') }}\n
            Hostname: {{hostname}}\n
            Network: {{network}}"
  - name: daemon
    monitorize: false
    # reload
    events: []
  - name: service
    monitorize: false
    # create, remove, update
    events: []
  - name: node
    monitorize: false
    # create, remove, update
    events: []
  - name: secret
    monitorize: false
    # create, remove, update
    events: []
  - name: config
    monitorize: false
    # create, remove, update
    events: []

publishers: ## Available publishers
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
      url: https://mm.tusitio.como
      token: xxxxxxxxxxxxxxxxxxxx
      channel_id: xxxxxxxxxxxxxxxxxxx
  - service: telegram
    enabled: false
    config:
      url: https://api.telegram.org
      token:
      chat_id:
  - service: zinc
    enabled: true
    config:
      url: https://zincobserve.tusitio.como
      token: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
      index: docker
  - service: matrix
    enabled: false
    config:
      url: matrix.tusitio.como
      token: xxxxxxxxxxxxxxxxxxxxxxxxxxx
      room: "!xxxxxxxxxxxxxxxxxxxxxxxx"
  - service: mosquitto
    enabled: true
    config:
      user: guest
      password: guest
      host: localhost
      port: 1883
      topic: bonzo/dog
  - service: rabbitmq
    enabled: false
    config:
      user: guest
      password: guest
      host: localhost
      port: 5672
      queue: docker
```

### Docker compose

If you want to use DEN with Docker Compose, this is an example,

```bash
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



##  Features

|    | Feature          | Description                                                                                                       |
|----|------------------|-------------------------------------------------------------------------------------------------------------------|
| ⚙️  | **Architecture** | The project utilizes a containerized architecture with Docker, enabling efficient deployment and scalability.      |
| 🔩 | **Code Quality** | The codebase maintains high quality standards with clear logic and adherence to Rust best practices.              |
| 📄 | **Documentation**| The project offers comprehensive documentation, detailing setup instructions, configuration, and usage guidelines.|
| 🔌 | **Integrations** | Key integrations include Docker for containerization and various messaging platforms for notifications.            |
| 🧩 | **Modularity**   | The codebase is structured in a modular fashion, promoting code reusability and ease of maintenance.               |
| 🧪 | **Testing**      | Testing frameworks and tools are not explicitly mentioned in the details provided for the project.                |
| ⚡️  | **Performance**  | The project aims to achieve efficiency and speed through its Rust implementation and Docker-based deployment.     |
| 🛡️ | **Security**     | Security measures may include Docker security features and access control mechanisms for monitoring configurations.|
| 📦 | **Dependencies** | Key dependencies include tokio, reqwest, serde for efficient Rust development, and various Docker-related packages.|
| 🚀 | **Scalability**  | The use of Docker and a containerized approach suggests good potential for scalability to handle increased loads.    |

---

##  Repository Structure

```sh
└── den/
    ├── Cargo.toml
    ├── Dockerfile
    ├── LICENSE
    ├── README.md
    ├── config.sample.yml
    ├── docker-compose.yml
    ├── entrypoint.sh
    ├── run.sh
    └── src
        ├── config.rs
        ├── error.rs
        ├── filters.rs
        ├── main.rs
        ├── object.rs
        └── publisher.rs
```

---

##  Modules

<details closed><summary>.</summary>

| File                                                                                | Summary                                                                                                                                                                                                                                                                                    |
| ---                                                                                 | ---                                                                                                                                                                                                                                                                                        |
| [run.sh](https://github.com/atareao/den/blob/master/run.sh)                         | Initiate containerized Den service using run.sh to facilitate environment setup with necessary configurations and components.                                                                                                                                                              |
| [docker-compose.yml](https://github.com/atareao/den/blob/master/docker-compose.yml) | Defines Docker service configuration for den image with restart policy, logging level, and volume mounts in the repository architecture.                                                                                                                                                   |
| [Cargo.toml](https://github.com/atareao/den/blob/master/Cargo.toml)                 | Implements dependencies in Cargo.toml for den project, managing packages like tokio, reqwest, serde for efficient Rust development.                                                                                                                                                        |
| [Dockerfile](https://github.com/atareao/den/blob/master/Dockerfile)                 | Builds a Rust application container using multi-stage Docker within the repositorys den' directory. Inherits required dependencies and sets up a compact Alpine-based image for deployment. Simplifies the distribution process by managing build artifacts efficiently.                   |
| [config.sample.yml](https://github.com/atareao/den/blob/master/config.sample.yml)   | Defines monitoring settings for various Docker objects with customizable event messages. Specifies publishers for notifications via Slack, Discord, Mattermost, Telegram, Zinc, Matrix, Mosquitto, and RabbitMQ, enhancing observability in the comprehensive Den repository architecture. |
| [entrypoint.sh](https://github.com/atareao/den/blob/master/entrypoint.sh)           | Initialize user and group, set ownership, and execute commands securely for the Den project through the entrypoint shell script.                                                                                                                                                           |

</details>

<details closed><summary>src</summary>

| File                                                                        | Summary                                                                                                                                                                                                                                                                      |
| ---                                                                         | ---                                                                                                                                                                                                                                                                          |
| [main.rs](https://github.com/atareao/den/blob/master/src/main.rs)           | Processes Docker events based on configuration, extracting relevant information and sending messages to enabled publishers. Loads configuration, initializes Docker, and listens for events. Logs event details, handles monitorization logic, and triggers message posting. |
| [error.rs](https://github.com/atareao/den/blob/master/src/error.rs)         | Defines a custom error struct with a message, implementing display and error traits for the repositorys error handling.                                                                                                                                                      |
| [filters.rs](https://github.com/atareao/den/blob/master/src/filters.rs)     | Implements a datetime formatting filter for Unix timestamps or ISO 8601 strings. Parses dates and times with timezone support. Takes formatting and timezone as arguments, defaults from context variables.                                                                  |
| [object.rs](https://github.com/atareao/den/blob/master/src/object.rs)       | Defines Docker event and object structs with parsing logic for event message templates and rendering based on event type. Handles different event scenarios with contextual data and filtering rules from the parent repositorys architecture.                               |
| [publisher.rs](https://github.com/atareao/den/blob/master/src/publisher.rs) | Implements publisher services for various platforms, enabling message posting via RabbitMQ, MQTT, Zinc, Telegram, and Mattermost. Posts messages utilizing specific configurations for each platform.                                                                        |
| [config.rs](https://github.com/atareao/den/blob/master/src/config.rs)       | Ability to check if monitoring is always enabled.                                                                                                                                                                                                                            |

</details>

---

##  Getting Started

**System Requirements:**

* **Rust**: `version x.y.z`

###  Installation

<h4>From <code>source</code></h4>

> 1. Clone the den repository:
>
> ```console
> $ git clone https://github.com/atareao/den
> ```
>
> 2. Change to the project directory:
> ```console
> $ cd den
> ```
>
> 3. Install the dependencies:
> ```console
> $ cargo build
> ```

###  Usage

<h4>From <code>source</code></h4>

> Run den using the command below:
> ```console
> $ cargo run
> ```

###  Tests

> Run the test suite using the command below:
> ```console
> $ cargo test
> ```

---

##  Project Roadmap

- [X] `► INSERT-TASK-1`
- [ ] `► INSERT-TASK-2`
- [ ] `► ...`

---

##  Contributing

Contributions are welcome! Here are several ways you can contribute:

- **[Report Issues](https://github.com/atareao/den/issues)**: Submit bugs found or log feature requests for the `den` project.
- **[Submit Pull Requests](https://github.com/atareao/den/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.
- **[Join the Discussions](https://github.com/atareao/den/discussions)**: Share your insights, provide feedback, or ask questions.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/atareao/den
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="center">
   <a href="https://github.com{/atareao/den/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=atareao/den">
   </a>
</p>
</details>

---

##  License

This project is protected under the [SELECT-A-LICENSE](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

[**Return**](#-overview)

---

