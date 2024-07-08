# den

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
