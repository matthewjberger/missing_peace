use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulateMode {
    Off,
    Fast,
    Slow,
}

impl SimulateMode {
    pub fn is_active(self) -> bool {
        self != SimulateMode::Off
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Act {
    Life,
    Grief,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayPhase {
    WakeUp,
    MandatoryChore,
    FreeTime,
    BedRitual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GriefPath {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Room {
    Bedroom,
    Kitchen,
    Living,
    Office,
    Porch,
    Backyard,
}

impl Room {
    pub fn display_name(&self) -> &'static str {
        match self {
            Room::Bedroom => "Bedroom",
            Room::Kitchen => "Kitchen",
            Room::Living => "Living Room",
            Room::Office => "Office",
            Room::Porch => "Porch",
            Room::Backyard => "Backyard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CareerPhase {
    Phase1,
    Phase2,
    Phase3,
    Phase4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DogAgePhase {
    Settled,
    Comfortable,
    Prime,
    Slowing,
    Fading,
    Final,
}

impl DogAgePhase {
    pub fn from_day(day: u32) -> Self {
        match day {
            1..=3 => DogAgePhase::Settled,
            4..=7 => DogAgePhase::Comfortable,
            8..=12 => DogAgePhase::Prime,
            13..=17 => DogAgePhase::Slowing,
            18..=19 => DogAgePhase::Fading,
            _ => DogAgePhase::Final,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DogPersonality {
    #[default]
    Playful,
    Gentle,
    Mischievous,
    Loyal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Weather {
    #[default]
    Sunny,
    Overcast,
    Rainy,
    Stormy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DogVisualStage {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
}

impl DogVisualStage {
    pub fn from_bond(bond: u32) -> Self {
        match bond {
            0..=10 => DogVisualStage::Stage1,
            11..=20 => DogVisualStage::Stage2,
            21..=30 => DogVisualStage::Stage3,
            31..=45 => DogVisualStage::Stage4,
            _ => DogVisualStage::Stage5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WifeMood {
    Happy,
    Content,
    Neutral,
    Lonely,
    Distant,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub occupation: String,
    pub favorite_color: String,
    pub favorite_food: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartnerInfo {
    pub name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DogInfo {
    pub name: String,
    #[serde(default)]
    pub personality: DogPersonality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub description: String,
    pub firsthand: bool,
    pub revisited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub day: u32,
    pub current_time: f64,
    pub act: Act,
    pub day_phase: DayPhase,
    pub current_room: Room,

    pub career: u32,
    pub home: u32,
    pub money: i32,
    pub energy: u32,
    pub peace: u32,

    pub bond: u32,
    pub partner_investment: u32,

    pub player: PlayerInfo,
    pub partner: PartnerInfo,
    pub dog: DogInfo,

    pub career_phase: CareerPhase,
    pub wife_mood: WifeMood,
    pub dog_fed_today: bool,
    pub dog_walked_today: bool,
    pub days_since_walk_invite: u32,

    pub grief_path: Option<GriefPath>,
    pub spiral_days: u32,
    pub fired: bool,
    pub is_ng_plus: bool,

    pub scenes_witnessed: HashSet<String>,
    pub memories: Vec<Memory>,

    pub major_project_days_remaining: u32,
    pub promoted: bool,

    pub recent_actions: Vec<String>,

    pub spiral_active: bool,
    pub chore_break_chance: f64,

    #[serde(default)]
    pub dog_moment_cooldown: u32,

    #[serde(default)]
    pub pre_grief_peace: u32,
    #[serde(default)]
    pub grief_day_count: u32,
    #[serde(default)]
    pub peace_floor: u32,
    #[serde(default)]
    pub turning_point_triggered: bool,
    #[serde(default)]
    pub path_b_effort_counter: u32,

    #[serde(default)]
    pub partner_actions_taken: Vec<String>,
    #[serde(default)]
    pub partner_milestone_reached: u32,
    #[serde(default)]
    pub date_night_count: u32,

    #[serde(default)]
    pub current_weather: Weather,

    #[serde(default)]
    pub previous_grief_path: Option<GriefPath>,
    #[serde(default)]
    pub echo_scenes_available: bool,

    #[serde(default)]
    pub has_dog_toy: bool,
    #[serde(default)]
    pub on_probation: bool,
    #[serde(default)]
    pub performance_review_pending: bool,
    #[serde(default)]
    pub career_warning_given: bool,
    #[serde(default)]
    pub keep_talking_available: bool,
    #[serde(default)]
    pub check_dog_today: u32,
    #[serde(default)]
    pub days_skipped: u32,
    #[serde(default)]
    pub major_project_uses: u32,
    #[serde(default)]
    pub cooked_today: bool,
    #[serde(default)]
    pub did_dishes_today: bool,
    #[serde(default)]
    pub did_laundry_today: bool,
    #[serde(default)]
    pub vacuumed_today: bool,
    #[serde(default)]
    pub shopped_today: bool,
    #[serde(default)]
    pub played_games_today: bool,
    #[serde(default)]
    pub checked_news_today: bool,
    #[serde(default)]
    pub petted_dog_today: bool,
    #[serde(default)]
    pub work_trip_taken: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            day: 1,
            current_time: 6.0,
            act: Act::Life,
            day_phase: DayPhase::WakeUp,
            current_room: Room::Bedroom,

            career: 0,
            home: 20,
            money: 50,
            energy: 4,
            peace: 50,

            bond: 0,
            partner_investment: 0,

            player: PlayerInfo::default(),
            partner: PartnerInfo::default(),
            dog: DogInfo::default(),

            career_phase: CareerPhase::Phase1,
            wife_mood: WifeMood::Content,
            dog_fed_today: false,
            dog_walked_today: false,
            days_since_walk_invite: 0,

            grief_path: None,
            spiral_days: 0,
            fired: false,
            is_ng_plus: false,

            scenes_witnessed: HashSet::new(),
            memories: Vec::new(),

            major_project_days_remaining: 0,
            promoted: false,

            recent_actions: Vec::new(),

            spiral_active: false,
            chore_break_chance: 0.0,

            dog_moment_cooldown: 0,

            pre_grief_peace: 0,
            grief_day_count: 0,
            peace_floor: 0,
            turning_point_triggered: false,
            path_b_effort_counter: 0,

            partner_actions_taken: Vec::new(),
            partner_milestone_reached: 0,
            date_night_count: 0,

            current_weather: Weather::Sunny,

            previous_grief_path: None,
            echo_scenes_available: false,

            has_dog_toy: false,
            on_probation: false,
            performance_review_pending: false,
            career_warning_given: false,
            keep_talking_available: false,
            check_dog_today: 0,
            days_skipped: 0,
            major_project_uses: 0,
            cooked_today: false,
            did_dishes_today: false,
            did_laundry_today: false,
            vacuumed_today: false,
            shopped_today: false,
            played_games_today: false,
            checked_news_today: false,
            petted_dog_today: false,
            work_trip_taken: false,
        }
    }
}

impl GameState {
    pub fn time_display(&self) -> String {
        let hour = self.current_time as u32;
        let minutes = ((self.current_time - hour as f64) * 60.0) as u32;
        let period = if hour >= 12 { "PM" } else { "AM" };
        let display_hour = if hour == 0 {
            12
        } else if hour > 12 {
            hour - 12
        } else {
            hour
        };
        format!("{}:{:02} {}", display_hour, minutes, period)
    }

    pub fn energy_display(&self) -> String {
        let filled = "\u{25C6}".repeat(self.energy as usize);
        let empty = "\u{25C7}".repeat(5_usize.saturating_sub(self.energy as usize));
        format!("{}{}", filled, empty)
    }

    pub fn dog_age_phase(&self) -> DogAgePhase {
        DogAgePhase::from_day(self.day)
    }

    pub fn dog_visual_stage(&self) -> DogVisualStage {
        DogVisualStage::from_bond(self.bond)
    }

    pub fn advance_time(&mut self, hours: f64) {
        self.current_time += hours;
    }

    pub fn is_bedtime(&self) -> bool {
        self.current_time >= 22.0
    }

    pub fn start_new_day(&mut self) {
        self.day += 1;
        self.current_time = 6.0;
        self.day_phase = DayPhase::WakeUp;

        if self.spiral_active {
            self.energy = 2;
        } else if self.act == Act::Grief && self.peace < 25 {
            self.energy = 3;
        } else {
            self.energy = 4;
        }

        self.dog_fed_today = false;
        self.dog_walked_today = false;
        self.current_room = Room::Bedroom;
        self.check_dog_today = 0;
        self.keep_talking_available = false;
        self.cooked_today = false;
        self.did_dishes_today = false;
        self.did_laundry_today = false;
        self.vacuumed_today = false;
        self.shopped_today = false;
        self.played_games_today = false;
        self.checked_news_today = false;
        self.petted_dog_today = false;

        if self.home > 0 {
            self.home = self.home.saturating_sub(1);
        }

        self.days_since_walk_invite += 1;
        self.recent_actions.clear();
    }
}
