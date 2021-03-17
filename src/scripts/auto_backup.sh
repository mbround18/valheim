#!/usr/bin/env bash
# Cron uses blank env and does not pick up /usr/local/bin files.
export PATH="/usr/local/bin:$PATH"
cd /home/steam/ || exit 1

log() {
  PREFIX="[Valheim][steam]"
  printf "%-16s: %s\n" "${PREFIX}" "$1"
}

log "Starting auto backup process..."

if [ "${AUTO_BACKUP_REMOVE_OLD:=0}" -eq 1 ]; then
    log "Removing old backups..."
    find /home/steam/backups -mtime +$((${AUTO_BACKUP_DAYS_TO_LIVE:-5} - 1)) -exec rm {} \;
fi

log "Creating backup..."
file_name="$(date +"%Y%m%d-%H%M%S")-${1:-"backup"}.tar.gz"

odin backup /home/steam/.config/unity3d/IronGate/Valheim "/home/steam/backups/${file_name}" || exit 1

log "Backup process complete! Created ${file_name}"
