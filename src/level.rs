use overworld::component::progress::ProgressBar;
use overworld::component::progression::experience::{
    ExperienceLevelUpTable, ExperienceSystem, ExperienceTracker,
};

pub fn experience() -> ExperienceSystem {
    ExperienceSystem::new(
        ExperienceTracker::new(),
        ExperienceLevelUpTable::new(vec![100, 10000, 20000, 30000]),
    )
}

pub fn current(score: u64) -> ExperienceSystem {
    let mut system = experience();
    system.add_experience(score);

    system
}

pub fn progress(system: &ExperienceSystem) -> ProgressBar {
    let mut pb = ProgressBar::new(system.get_next_milestone().unwrap_or(0));
    pb.set_progress(system.get_experience() as f64);

    pb
}
