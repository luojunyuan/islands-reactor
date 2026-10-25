use windows_core::Result;

use crate::AccessibilityModifiers;
use crate::bindings::{
    AutomationHeadingLevel, AutomationLiveSetting, AutomationProperties, DependencyObject,
};

pub(super) fn apply(
    element: &DependencyObject,
    accessibility: &AccessibilityModifiers,
) -> Result<()> {
    AutomationProperties::SetName(
        element,
        accessibility.automation_name.as_deref().unwrap_or_default(),
    )?;
    AutomationProperties::SetAutomationId(
        element,
        accessibility.automation_id.as_deref().unwrap_or_default(),
    )?;
    AutomationProperties::SetHelpText(
        element,
        accessibility.help_text.as_deref().unwrap_or_default(),
    )?;
    AutomationProperties::SetLiveSetting(element, live_setting(accessibility.live_setting))?;
    AutomationProperties::SetHeadingLevel(element, heading_level(accessibility.heading_level))
}

fn live_setting(value: Option<crate::LiveSetting>) -> AutomationLiveSetting {
    match value {
        Some(crate::LiveSetting::Polite) => AutomationLiveSetting::Polite,
        Some(crate::LiveSetting::Assertive) => AutomationLiveSetting::Assertive,
        Some(crate::LiveSetting::Off) | None => AutomationLiveSetting::Off,
    }
}

fn heading_level(value: Option<crate::HeadingLevel>) -> AutomationHeadingLevel {
    match value {
        Some(crate::HeadingLevel::Level1) => AutomationHeadingLevel::Level1,
        Some(crate::HeadingLevel::Level2) => AutomationHeadingLevel::Level2,
        Some(crate::HeadingLevel::Level3) => AutomationHeadingLevel::Level3,
        Some(crate::HeadingLevel::Level4) => AutomationHeadingLevel::Level4,
        Some(crate::HeadingLevel::Level5) => AutomationHeadingLevel::Level5,
        Some(crate::HeadingLevel::Level6) => AutomationHeadingLevel::Level6,
        Some(crate::HeadingLevel::Level7) => AutomationHeadingLevel::Level7,
        Some(crate::HeadingLevel::Level8) => AutomationHeadingLevel::Level8,
        Some(crate::HeadingLevel::Level9) => AutomationHeadingLevel::Level9,
        None => AutomationHeadingLevel::None,
    }
}
