#!/usr/bin/env bash
# Cron uses blank env and does not pick up /usr/local/bin files.
export PATH="/usr/local/bin:$PATH"

log() {
  PREFIX="[Valheim][steam]"
  printf "%-16s: %s\n" "${PREFIX}" "$1"
}
line () {
  log "###########################################################################"
}

line
log "Valheim Server - $(date)"

cd /home/steam/valheim || exit 1

if odin update --check; then
    log "An update is available. Starting the update process..."

    # Store if the server is currently running
    ! pidof valheim_server.x86_64 > /dev/null
    SERVER_RUNNING=$?

    # Stop the server if it's running
    if [ "${SERVER_RUNNING}" -eq 1 ]; then
        odin stop || exit 1
    fi

    if [ "${AUTO_BACKUP_ON_UPDATE:=0}" -eq 1 ]; then
        /bin/bash /home/steam/scripts/auto_backup.sh "pre-update-backup"
    fi

    odin update || exit 1

    # Start the server if it was running before
    if [ "${SERVER_RUNNING}" -eq 1 ]; then
        odin start || exit 1
        line
        log "
        Finished updating and everything looks happy <3

        Check your output.log for 'Game server connected'
        "
    fi
else
    log "No update available"
fi

line
