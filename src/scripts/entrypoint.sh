#!/usr/bin/env bash
cd /home/steam/valheim || exit 1
STEAM_UID=${PUID:=1000}
STEAM_GID=${PGID:=1000}

# Configure ENV
. /home/steam/scripts/load_env.sh

initialize () {
  echo "
###########################################################################
Valheim Server - $(date)
STEAM_UID ${STEAM_UID} - STEAM_GUID ${STEAM_GID}

$1

###########################################################################
  "
}

log () {
  echo "[Valheim][steam]: $1"
}

initialize "Installing Valheim via Odin..."

export SteamAppId=892970
export PATH="/home/steam/.odin:$PATH"

# Setting up server
log "Running Install..."
odin install || exit 1

log "Herding Cats..."
log "Starting server..."

odin start || exit 1


cleanup() {
    log "Halting server! Received interrupt!"
    odin stop
    exit
}
trap 'cleanup' INT TERM EXIT SIGINT SIGTERM

initialize "
Valheim Server Started...

Keep an eye out for 'Game server connected' in the log!
(this indicates its online without any errors.)
" >> /home/steam/valheim/output.log
tail -f /home/steam/valheim/output.log

while :; do
  sleep 1s
done
