use crate::{
    config::{BackupFormat, RedirectKind, RootsConfig, SortKey, Theme, ZipCompression},
    gui::{badge::Badge, icon::Icon},
    lang::{Language, Translator},
    layout::{Backup, GameLayout},
    manifest::{ManifestUpdate, Store},
    prelude::{BackupInfo, Error, OperationStatus, OperationStepDecision, RegistryItem, ScanInfo, StrictPath},
    shortcuts::{Shortcut, TextHistory},
};

use crate::gui::widget::{Button, Column, Element, Row, Text};
use iced::{Alignment, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    Error(Error),
    Exit,
    CloseModal,
    PruneNotifications,
    UpdateManifest,
    ManifestUpdated(Result<Option<ManifestUpdate>, Error>),
    ConfirmBackupStart {
        games: Option<Vec<String>>,
    },
    BackupPrep {
        preview: bool,
        games: Option<Vec<String>>,
    },
    BackupStart {
        preview: bool,
        games: Option<Vec<String>>,
    },
    ConfirmRestoreStart {
        games: Option<Vec<String>>,
    },
    RestoreStart {
        preview: bool,
        games: Option<Vec<String>>,
    },
    BackupStep {
        scan_info: Option<ScanInfo>,
        backup_info: Option<BackupInfo>,
        decision: OperationStepDecision,
        preview: bool,
        full: bool,
    },
    RestoreStep {
        scan_info: Option<ScanInfo>,
        backup_info: Option<BackupInfo>,
        decision: OperationStepDecision,
        full: bool,
        game_layout: GameLayout,
    },
    CancelOperation,
    EditedBackupTarget(String),
    EditedBackupMerge(bool),
    EditedRestoreSource(String),
    FindRoots,
    ConfirmAddMissingRoots(Vec<RootsConfig>),
    EditedRoot(EditAction),
    SelectedRootStore(usize, Store),
    SelectedRedirectKind(usize, RedirectKind),
    EditedRedirect(EditAction, Option<RedirectEditActionField>),
    EditedCustomGame(EditAction),
    EditedCustomGameFile(usize, EditAction),
    EditedCustomGameRegistry(usize, EditAction),
    EditedExcludeStoreScreenshots(bool),
    EditedBackupFilterIgnoredPath(EditAction),
    EditedBackupFilterIgnoredRegistry(EditAction),
    SwitchScreen(Screen),
    ToggleGameListEntryExpanded {
        name: String,
    },
    ToggleGameListEntryTreeExpanded {
        name: String,
        keys: Vec<TreeNodeKey>,
    },
    ToggleGameListEntryEnabled {
        name: String,
        enabled: bool,
        restoring: bool,
    },
    ToggleSearch {
        screen: Screen,
    },
    ToggleSpecificBackupPathIgnored {
        name: String,
        path: StrictPath,
        enabled: bool,
    },
    ToggleSpecificBackupRegistryIgnored {
        name: String,
        path: RegistryItem,
        value: Option<String>,
        enabled: bool,
    },
    ToggleCustomGameEnabled {
        index: usize,
        enabled: bool,
    },
    EditedSearchGameName {
        screen: Screen,
        value: String,
    },
    EditedSortKey {
        screen: Screen,
        value: SortKey,
    },
    EditedSortReversed {
        screen: Screen,
        value: bool,
    },
    BrowseDir(BrowseSubject),
    BrowseDirFailure,
    SelectAllGames,
    DeselectAllGames,
    OpenDir {
        path: StrictPath,
    },
    OpenDirFailure {
        path: StrictPath,
    },
    OpenUrlFailure {
        url: String,
    },
    KeyboardEvent(iced_native::keyboard::Event),
    EditedFullRetention(u8),
    EditedDiffRetention(u8),
    SelectedBackupToRestore {
        game: String,
        backup: Backup,
    },
    SelectedLanguage(Language),
    SelectedTheme(Theme),
    SelectedBackupFormat(BackupFormat),
    SelectedBackupCompression(ZipCompression),
    EditedCompressionLevel(i32),
    ToggleBackupSettings,
    GameAction {
        action: GameAction,
        game: String,
    },
    UndoRedo(crate::gui::undoable::Action, UndoSubject),
    Scroll {
        subject: ScrollSubject,
        position: iced_native::widget::scrollable::RelativeOffset,
    },
    EditedBackupComment {
        game: String,
        comment: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OngoingOperation {
    Backup,
    CancelBackup,
    PreviewBackup,
    CancelPreviewBackup,
    Restore,
    CancelRestore,
    PreviewRestore,
    CancelPreviewRestore,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Backup,
    Restore,
    CustomGames,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditDirection {
    Up,
    Down,
}

impl EditDirection {
    pub fn shift(&self, index: usize) -> usize {
        match self {
            Self::Up => index - 1,
            Self::Down => index + 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditAction {
    Add,
    Change(usize, String),
    Remove(usize),
    Move(usize, EditDirection),
}

impl EditAction {
    pub fn move_up(index: usize) -> Self {
        Self::Move(index, EditDirection::Up)
    }

    pub fn move_down(index: usize) -> Self {
        Self::Move(index, EditDirection::Down)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectEditActionField {
    Source,
    Target,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowseSubject {
    BackupTarget,
    RestoreSource,
    Root(usize),
    RedirectSource(usize),
    RedirectTarget(usize),
    CustomGameFile(usize, usize),
    BackupFilterIgnoredPath(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UndoSubject {
    BackupTarget,
    RestoreSource,
    BackupSearchGameName,
    RestoreSearchGameName,
    Root(usize),
    RedirectSource(usize),
    RedirectTarget(usize),
    CustomGameName(usize),
    CustomGameFile(usize, usize),
    CustomGameRegistry(usize, usize),
    BackupFilterIgnoredPath(usize),
    BackupFilterIgnoredRegistry(usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScrollSubject {
    Backup,
    Restore,
    CustomGames,
    Other,
    Modal,
}

impl ScrollSubject {
    pub fn game_list(restoring: bool) -> Self {
        if restoring {
            Self::Restore
        } else {
            Self::Backup
        }
    }

    pub fn id(&self) -> iced_native::widget::scrollable::Id {
        match self {
            Self::Backup => crate::gui::widget::id::backup_scroll(),
            Self::Restore => crate::gui::widget::id::restore_scroll(),
            Self::CustomGames => crate::gui::widget::id::custom_games_scroll(),
            Self::Other => crate::gui::widget::id::other_scroll(),
            Self::Modal => crate::gui::widget::id::modal_scroll(),
        }
    }

    pub fn into_widget<'a>(
        self,
        content: impl Into<crate::gui::widget::Element<'a>>,
    ) -> crate::gui::widget::Scrollable<'a> {
        crate::gui::widget::Scrollable::new(content)
            .height(Length::Fill)
            .style(crate::gui::style::Scrollable)
            .id(self.id())
            .on_scroll(move |position| Message::Scroll {
                subject: self,
                position,
            })
    }
}

impl From<Screen> for ScrollSubject {
    fn from(value: Screen) -> Self {
        match value {
            Screen::Backup => Self::Backup,
            Screen::Restore => Self::Restore,
            Screen::CustomGames => Self::CustomGames,
            Screen::Other => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    Customize,
    PreviewBackup,
    Backup { confirm: bool },
    PreviewRestore,
    Restore { confirm: bool },
    Wiki,
    Comment,
}

impl GameAction {
    pub fn options(restoring: bool, operating: bool, customized: bool, invented: bool, has_backups: bool) -> Vec<Self> {
        let mut options = vec![];

        if !operating {
            if restoring {
                options.push(Self::PreviewRestore);
                options.push(Self::Restore { confirm: true });
            } else {
                options.push(Self::PreviewBackup);
                options.push(Self::Backup { confirm: true });
            }
        }

        if !restoring && !customized {
            options.push(Self::Customize);
        }

        if restoring && has_backups {
            options.push(Self::Comment);
        }

        if !invented {
            options.push(Self::Wiki);
        }

        options
    }

    pub fn icon(&self) -> Icon {
        match self {
            GameAction::Backup { confirm } | GameAction::Restore { confirm } => {
                if *confirm {
                    Icon::PlayCircleOutline
                } else {
                    Icon::FastForward
                }
            }
            GameAction::PreviewBackup | GameAction::PreviewRestore => Icon::Refresh,
            GameAction::Customize => Icon::Edit,
            GameAction::Wiki => Icon::Language,
            GameAction::Comment => Icon::Comment,
        }
    }
}

impl ToString for GameAction {
    fn to_string(&self) -> String {
        let translator = Translator::default();
        match self {
            Self::PreviewBackup | Self::PreviewRestore => translator.preview_button(),
            Self::Backup { confirm } => {
                if *confirm {
                    translator.backup_button()
                } else {
                    translator.backup_button_no_confirmation()
                }
            }
            Self::Restore { confirm } => {
                if *confirm {
                    translator.restore_button()
                } else {
                    translator.restore_button_no_confirmation()
                }
            }
            Self::Customize => translator.customize_button(),
            Self::Wiki => translator.pcgamingwiki(),
            Self::Comment => translator.comment_button(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TreeNodeKey {
    File(String),
    RegistryKey(String),
    RegistryValue(String),
}

impl TreeNodeKey {
    pub fn raw(&self) -> &str {
        match self {
            Self::File(x) => x,
            Self::RegistryKey(x) => x,
            Self::RegistryValue(x) => x,
        }
    }
}

pub fn apply_shortcut_to_strict_path_field(shortcut: &Shortcut, config: &mut StrictPath, history: &mut TextHistory) {
    match shortcut {
        Shortcut::Undo => {
            config.reset(history.undo());
        }
        Shortcut::Redo => {
            config.reset(history.redo());
        }
    }
}

pub fn apply_shortcut_to_registry_path_field(
    shortcut: &Shortcut,
    config: &mut RegistryItem,
    history: &mut TextHistory,
) {
    match shortcut {
        Shortcut::Undo => {
            config.reset(history.undo());
        }
        Shortcut::Redo => {
            config.reset(history.redo());
        }
    }
}

pub fn apply_shortcut_to_string_field(shortcut: &Shortcut, config: &mut String, history: &mut TextHistory) {
    match shortcut {
        Shortcut::Undo => {
            *config = history.undo();
        }
        Shortcut::Redo => {
            *config = history.redo();
        }
    }
}

pub fn make_status_row<'a>(translator: &Translator, status: &OperationStatus, found_any_duplicates: bool) -> Row<'a> {
    Row::new()
        .padding([0, 20, 0, 20])
        .align_items(Alignment::Center)
        .spacing(15)
        .push(Text::new(translator.processed_games(status)).size(35))
        .push_if(
            || status.changed_games.new > 0,
            || Badge::new_entry_with_count(translator, status.changed_games.new).view(),
        )
        .push_if(
            || status.changed_games.different > 0,
            || Badge::changed_entry_with_count(translator, status.changed_games.different).view(),
        )
        .push(Text::new("|").size(35))
        .push(Text::new(translator.processed_bytes(status)).size(35))
        .push_if(
            || found_any_duplicates,
            || Badge::new(&translator.badge_duplicates()).view(),
        )
}

pub trait IcedExtension<'a> {
    fn push_if<E>(self, condition: impl FnOnce() -> bool, element: impl FnOnce() -> E) -> Self
    where
        E: Into<Element<'a>>;

    fn push_some<E>(self, element: impl FnOnce() -> Option<E>) -> Self
    where
        E: Into<Element<'a>>;
}

impl<'a> IcedExtension<'a> for Column<'a> {
    fn push_if<E>(self, condition: impl FnOnce() -> bool, element: impl FnOnce() -> E) -> Self
    where
        E: Into<Element<'a>>,
    {
        if condition() {
            self.push(element().into())
        } else {
            self
        }
    }

    fn push_some<E>(self, element: impl FnOnce() -> Option<E>) -> Self
    where
        E: Into<Element<'a>>,
    {
        if let Some(element) = element() {
            self.push(element.into())
        } else {
            self
        }
    }
}

impl<'a> IcedExtension<'a> for Row<'a> {
    fn push_if<E>(self, condition: impl FnOnce() -> bool, element: impl FnOnce() -> E) -> Self
    where
        E: Into<Element<'a>>,
    {
        if condition() {
            self.push(element().into())
        } else {
            self
        }
    }

    fn push_some<E>(self, element: impl FnOnce() -> Option<E>) -> Self
    where
        E: Into<Element<'a>>,
    {
        if let Some(element) = element() {
            self.push(element.into())
        } else {
            self
        }
    }
}

pub trait IcedButtonExt<'a> {
    fn on_press_if(self, condition: impl FnOnce() -> bool, msg: impl FnOnce() -> Message) -> Self;
    fn on_press_some(self, msg: Option<Message>) -> Self;
}

impl<'a> IcedButtonExt<'a> for Button<'a> {
    fn on_press_if(self, condition: impl FnOnce() -> bool, msg: impl FnOnce() -> Message) -> Self {
        if condition() {
            self.on_press(msg())
        } else {
            self
        }
    }

    fn on_press_some(self, msg: Option<Message>) -> Self {
        match msg {
            Some(msg) => self.on_press(msg),
            None => self,
        }
    }
}
