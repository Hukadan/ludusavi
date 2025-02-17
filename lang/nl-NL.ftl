ludusavi = Ludusavi
language = Taal
font = Lettertype
game-name = Naam
total-games = Games
file-size = Grootte
file-location = Locatie
overall = Totaal
cli-unrecognized-games = Geen informatie voor deze games:
cli-unable-to-request-confirmation = Er kan niet om bevestiging worden gevraagd.
    .winpty-workaround = Als je een Bash-emulator, zoals Git Bash, gebruikt, probeer dan winpty uit te voeren.
cli-backup-id-with-multiple-games = De back-up-id kan niet worden opgegeven bij het herstellen van meerdere games.
cli-invalid-backup-id = Ongeldige back-up-id.
badge-failed = MISLUKT
badge-duplicates = DUPLICATEN
badge-duplicated = GEDUPLICEERD
badge-ignored = GENEGEERD
badge-redirected-from = VAN: { $path }
badge-redirecting-to = NAAR: { $path }
some-entries-failed = Sommige items konden niet worden verwerkt - zoek naar { badge-failed } in de uitvoer om de details te bekijken. Controleer nogmaals of je toegang hebt tot deze bestanden of of hun paden erg lang zijn.
cli-game-line-item-redirected = Doorverwezen van: { $path }
cli-game-line-item-redirecting = Doorverwijzen naar: { $path }
button-backup = Back-up
button-preview = Voorvertoning
button-restore = Herstellen
button-nav-backup = BACK-UPMODUS
button-nav-restore = HERSTELMODUS
button-nav-custom-games = ANDERE SPELLEN
button-nav-other = OVERIG
button-add-game = Game toevoegen
button-continue = Doorgaan
button-cancel = Annuleren
button-cancelling = Bezig met annuleren…
button-okay = Oké
button-select-all = Alles selecteren
button-deselect-all = Niets selecteren
button-enable-all = Alles inschakelen
button-disable-all = Alles uitschakelen
button-customize = Aanpassen
button-exit = Afsluiten
button-comment = Comment
no-roots-are-configured = Voeg hoofdmappen toe om meer gegevens te back-uppen.
config-is-invalid = Foutmelding: het configuratiebestand is ongeldig.
manifest-is-invalid = Foutmelding: het manifestbestand is ongeldig.
manifest-cannot-be-updated = Foutmelding: er kan niet worden gecontroleerd op een update van het manifestbestand. Ben je verbonden met het internet?
cannot-prepare-backup-target = Error: Unable to prepare backup target (either creating or emptying the folder). If you have the folder open in your file browser, try closing it: { $path }
restoration-source-is-invalid = Error: The restoration source is invalid (either doesn't exist or isn't a directory). Please double check the location: { $path }
registry-issue = Error: Some registry entries were skipped.
unable-to-browse-file-system = Error: Unable to browse on your system.
unable-to-open-directory = Error: Unable to open directory:
unable-to-open-url = Error: Unable to open URL:
processed-games =
    { $total-games } { $total-games ->
        [one] game
       *[other] games
    }
processed-games-subset =
    { $processed-games } of { $total-games } { $total-games ->
        [one] game
       *[other] games
    }
processed-size-subset = { $processed-size } of { $total-size }
field-backup-target = Back up to:
toggle-backup-merge = Merge
field-restore-source = Restore from:
field-custom-files = Paths:
field-custom-registry = Registry:
field-search = Search:
field-sort = Sort:
field-redirect-source =
    .placeholder = Source (original location)
field-redirect-target =
    .placeholder = Target (new location)
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
