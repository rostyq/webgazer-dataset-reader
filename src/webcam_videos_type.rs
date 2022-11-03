pub enum WebcamVideoType {
    DotTestInstructions,
    DotTest,
    FittsLawInstructions,
    FittsLaw,
    SerpInstructions,
    BenefitsOfRunningInstructions,
    BenefitsOfRunning,
    BenefitsOfRunningWriting,
    EducationalAdvantagesOfSocialNetworkingSitesInstructions,
    EducationalAdvantagesOfSocialNetworkingSites,
    BeducationalAdvantagesOfSocialNetworkingSitesWriting,
    WhereToFindMorelMushroomsInstructions,
    WhereToFindMorelMushrooms,
    WhereToFindMorelMushroomsWriting,
    ToothAbscessInstructions,
    ToothAbscess,
    ToothAbscessWriting,
    DotTestFinalInstructions,
    DotTestFinal,
    ThankYou,
}

impl WebcamVideoType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DotTestInstructions => "dot_test_instructions",
            Self::DotTest => "dot_test",
            Self::FittsLawInstructions => "fitts_law_instructions",
            Self::FittsLaw => "fitts_law",
            Self::SerpInstructions => "serp_instructions",
            Self::BenefitsOfRunningInstructions => "benefits_of_running_instructions",
            Self::BenefitsOfRunning => "benefits_of_running",
            Self::BenefitsOfRunningWriting => "benefits_of_running_writing",
            Self::EducationalAdvantagesOfSocialNetworkingSitesInstructions => {
                "educational_advantages_of_social_networking_sites_instructions"
            }
            Self::EducationalAdvantagesOfSocialNetworkingSites => {
                "educational_advantages_of_social_networking_sites"
            }
            Self::BeducationalAdvantagesOfSocialNetworkingSitesWriting => {
                "beducational_advantages_of_social_networking_sites_writing"
            }
            Self::WhereToFindMorelMushroomsInstructions => {
                "where_to_find_morel_mushrooms_instructions"
            }
            Self::WhereToFindMorelMushrooms => "where_to_find_morel_mushrooms",
            Self::WhereToFindMorelMushroomsWriting => "where_to_find_morel_mushrooms_writing",
            Self::ToothAbscessInstructions => "tooth_abscess_instructions",
            Self::ToothAbscess => "tooth_abscess",
            Self::ToothAbscessWriting => "tooth_abscess_writing",
            Self::DotTestFinalInstructions => "dot_test_final_instructions",
            Self::DotTestFinal => "dot_test_final",
            Self::ThankYou => "thank_you",
        }
    }
}

impl ToString for WebcamVideoType {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

impl From<&str> for WebcamVideoType {
    fn from(p: &str) -> Self {
        match p {
            "dot_test_instructions" => Self::DotTestInstructions,
            "dot_test" => Self::DotTest,
            "fitts_law_instructions" => Self::FittsLawInstructions,
            "fitts_law" => Self::FittsLaw,
            "serp_instructions" => Self::SerpInstructions,
            "benefits_of_running_instructions" => Self::BenefitsOfRunningInstructions,
            "benefits_of_running" => Self::BenefitsOfRunning,
            "benefits_of_running_writing" => Self::BenefitsOfRunningWriting,
            "educational_advantages_of_social_networking_sites_instructions" => {
                Self::EducationalAdvantagesOfSocialNetworkingSitesInstructions
            }
            "educational_advantages_of_social_networking_sites" => {
                Self::EducationalAdvantagesOfSocialNetworkingSites
            }
            "beducational_advantages_of_social_networking_sites_writing" => {
                Self::BeducationalAdvantagesOfSocialNetworkingSitesWriting
            }
            "where_to_find_morel_mushrooms_instructions" => {
                Self::WhereToFindMorelMushroomsInstructions
            }
            "where_to_find_morel_mushrooms" => Self::WhereToFindMorelMushrooms,
            "where_to_find_morel_mushrooms_writing" => Self::WhereToFindMorelMushroomsWriting,
            "tooth_abscess_instructions" => Self::ToothAbscessInstructions,
            "tooth_abscess" => Self::ToothAbscess,
            "tooth_abscess_writing" => Self::ToothAbscessWriting,
            "dot_test_final_instructions" => Self::DotTestFinalInstructions,
            "dot_test_final" => Self::DotTestFinal,
            "thank_you" => Self::ThankYou,
        }
    }
}
