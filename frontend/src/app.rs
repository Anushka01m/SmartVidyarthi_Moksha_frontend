use leptos::*;
use leptos_router::*;

use crate::components::{
    home::Home, login::Login, register::Register, dashboard::Dashboard,
    leaderboard::Leaderboard, achievements::Achievements, mascot_switcher::MascotSwitcher,
    mentor::Mentor, lesson_plan::LessonPlan, lesson_detail::LessonDetail,
    quiz::Quiz, assignments::Assignments, community::CommunitySuggestions, voting::Voting,
    faq_chat::FaqChat, teacher_dashboard::TeacherDashboard, admin_panel::AdminPanel,
    settings::Settings, theme_switcher::ThemeSwitcher
};

pub fn view() -> impl IntoView {
    view! {
        <Router>
            <Route path="/" view=Home />
            <Route path="/login" view=Login />
            <Route path="/register" view=Register />
            <Route path="/dashboard" view=Dashboard />
            <Route path="/leaderboard" view=Leaderboard />
            <Route path="/achievements" view=Achievements />
            <Route path="/mascot_switcher" view=MascotSwitcher />
            <Route path="/mentor" view=Mentor />
            <Route path="/plan" view=LessonPlan />
            <Route path="/lesson/:id" view=LessonDetail />
            <Route path="/quiz" view=Quiz />
            <Route path="/assignments" view=Assignments />
            <Route path="/community" view=CommunitySuggestions />
            <Route path="/voting" view=Voting />
            <Route path="/faq_chat" view=FaqChat />
            <Route path="/teacher_dashboard" view=TeacherDashboard />
            <Route path="/admin_panel" view=AdminPanel />
            <Route path="/settings" view=Settings />
            <Route path="/theme_switcher" view=ThemeSwitcher />
        </Router>
    }
}
