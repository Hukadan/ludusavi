ludusavi = Ludusavi
language = Wika
font = Font
game-name = Pamagat
total-games = Games
file-size = Sukat
file-location = Lokasyon
overall = Buod
cli-unrecognized-games = Wala pang impormasyon para sa mga games na ito:
cli-unable-to-request-confirmation = Hindi makahiling ng kumpirmasyon.
    .winpty-workaround = Pag ginagamit mo ng Bash emulator (halimbawa Git Bash), subukin mo gamitin ng winpty.
cli-backup-id-with-multiple-games = Cannot specify backup ID when restoring multiple games.
cli-invalid-backup-id = Invalid backup ID.
badge-failed = BUMAGSAK
badge-duplicates = KAPAREHO
badge-duplicated = KINOPYA
badge-ignored = DEDMA
badge-redirected-from = MULA SA: { $path }
badge-redirecting-to = TO: { $path }
some-entries-failed = Mayroon mali sa proseso; hanapin mo ng { badge-failed } sa output. Paki-tiyak kung kaya mong buksan ang mga files o kung masyado mahaba ang mga paths.
cli-game-line-item-redirected = Na-redirect mula sa: { $path }
cli-game-line-item-redirecting = Redirecting to: { $path }
button-backup = Back up
button-preview = Preview
button-restore = Restore
button-nav-backup = BACKUP MODE
button-nav-restore = RESTORE MODE
button-nav-custom-games = PASADYANG GAMES
button-nav-other = ATBP
button-add-game = Lagyan ng game
button-continue = Tuloy
button-cancel = Alisin
button-cancelling = Inaalis...
button-okay = Sige
button-select-all = Piliin ang lahat
button-deselect-all = Alisin ang lahat
button-enable-all = Enable ang lahat
button-disable-all = Disable ang lahat
button-customize = Customize
button-exit = Exit
button-comment = Comment
no-roots-are-configured = Paki-lagay mga roots upang mag-back up ng higit pang data.
config-is-invalid = Error: Invalid ang config file.
manifest-is-invalid = Error: Invalid ang manifest file.
manifest-cannot-be-updated = Error: Hindi masuri kung may update sa manifest file. Nawala ba ang Internet connection niyo?
cannot-prepare-backup-target = Error: Hindi maihanda ang backup target (alinman sa paggawa o pag-alis ng laman sa folder). Kung nakabukas ang folder sa iyong file browser, subukang isara ito: { $path }
restoration-source-is-invalid = Error: Invalid ang restoration source (alinman sa hindi siya umiiral o hindi siya directory). Paki-tiyak ang lokasyon: { $path }
registry-issue = Error: May mga registry entries nilakdawan.
unable-to-browse-file-system = Error: Hindi mabuksan ang file browser sa iyong system.
unable-to-open-directory = Error: Hindi mabuksan ang directory:
unable-to-open-url = Error: Hindi mabuksan ang URL:
processed-games =
    { $total-games } { $total-games ->
        [one] game
       *[other] games
    }
processed-games-subset =
    { $processed-games } sa { $total-games } { $total-games ->
        [one] game
       *[other] games
    }
processed-size-subset = { $processed-size } sa { $total-size }
field-backup-target = Back up sa:
toggle-backup-merge = Pagsamahin
field-restore-source = Ibalik mula sa:
field-custom-files = Paths:
field-custom-registry = Registry:
field-search = Hanapin:
field-sort = Sort:
field-redirect-source =
    .placeholder = Source (orihinal na lokasyon)
field-redirect-target =
    .placeholder = Target (bagong lokasyon)
field-roots = Roots:
field-backup-excluded-items = Backup exclusions:
field-redirects = Redirects:
# This appears next to the number of full backups that you'd like to keep.
# A full backup includes all save files for a game.
field-retention-full = Full:
# This appears next to the number of differential backups that you'd like to keep.
# A differential backup includes only the files that have changed since the last full backup.
field-retention-differential = Differential:
field-backup-format = Format:
field-backup-compression = Compression:
# The compression level determines how much compresison we perform.
field-backup-compression-level = Level:
label-manifest = Manifest
# This shows the time when we checked for an update to the manifest.
label-checked = Checked
# This shows the time when we found an update to the manifest.
label-updated = Updated
label-new = New
label-comment = Comment
store-epic = Epic
store-gog = GOG
store-gog-galaxy = GOG Galaxy
store-heroic = Heroic
store-microsoft = Microsoft
store-origin = Origin
store-prime = Prime Gaming
store-steam = Steam
store-uplay = Uplay
store-other-home = Home folder
store-other-wine = Wine prefix
store-other = Other
sort-reversed = Reversed
backup-format-simple = Simple
backup-format-zip = Zip
compression-none = None
# "Deflate" is a proper noun: https://en.wikipedia.org/wiki/Deflate
compression-deflate = Deflate
compression-bzip2 = Bzip2
compression-zstd = Zstd
theme = Theme
theme-light = Light
theme-dark = Dark
redirect-bidirectional = Bidirectional
explanation-for-exclude-store-screenshots =
    In backups, exclude store-specific screenshots. Right now, this only applies
    to { store-steam } screenshots that you've taken. If a game has its own built-in
    screenshot functionality, this setting will not affect whether those
    screenshots are backed up.
consider-doing-a-preview =
    If you haven't already, consider doing a preview first so that there
    are no surprises.
confirm-backup =
    Are you sure you want to proceed with the backup? { $path-action ->
        [merge] New save data will be merged into the target folder:
        [recreate] The target folder will be deleted and recreated from scratch:
       *[create] The target folder will be created:
    }
confirm-restore =
    Are you sure you want to proceed with the restoration?
    This will overwrite any current files with the backups from here:
confirm-add-missing-roots = Add these roots?
no-missing-roots = No additional roots found.
preparing-backup-target = Preparing backup directory...
updating-manifest = Updating manifest...
saves-found = Save data found.
no-saves-found = No save data found.
# This is tacked on to form something like "Back up (no confirmation)",
# meaning we would perform an action without asking the user if they're sure.
suffix-no-confirmation = no confirmation
