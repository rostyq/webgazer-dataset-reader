#[derive(Debug, Clone, Copy)]
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

impl TryFrom<&str> for WebcamVideoType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dot_test_instructions" => Ok(Self::DotTestInstructions),
            "dot_test" => Ok(Self::DotTest),
            "fitts_law_instructions" => Ok(Self::FittsLawInstructions),
            "fitts_law" => Ok(Self::FittsLaw),
            "serp_instructions" => Ok(Self::SerpInstructions),
            "benefits_of_running_instructions" => Ok(Self::BenefitsOfRunningInstructions),
            "benefits_of_running" => Ok(Self::BenefitsOfRunning),
            "benefits_of_running_writing" => Ok(Self::BenefitsOfRunningWriting),
            "educational_advantages_of_social_networking_sites_instructions" => {
                Ok(Self::EducationalAdvantagesOfSocialNetworkingSitesInstructions)
            }
            "educational_advantages_of_social_networking_sites" => {
                Ok(Self::EducationalAdvantagesOfSocialNetworkingSites)
            }
            "beducational_advantages_of_social_networking_sites_writing" => {
                Ok(Self::BeducationalAdvantagesOfSocialNetworkingSitesWriting)
            }
            "where_to_find_morel_mushrooms_instructions" => {
                Ok(Self::WhereToFindMorelMushroomsInstructions)
            }
            "where_to_find_morel_mushrooms" => Ok(Self::WhereToFindMorelMushrooms),
            "where_to_find_morel_mushrooms_writing" => Ok(Self::WhereToFindMorelMushroomsWriting),
            "tooth_abscess_instructions" => Ok(Self::ToothAbscessInstructions),
            "tooth_abscess" => Ok(Self::ToothAbscess),
            "tooth_abscess_writing" => Ok(Self::ToothAbscessWriting),
            "dot_test_final_instructions" => Ok(Self::DotTestFinalInstructions),
            "dot_test_final" => Ok(Self::DotTestFinal),
            "thank_you" => Ok(Self::ThankYou),
            unsupported_type_name => Err(format!("Unsupported webcam video type name: `{}`", unsupported_type_name)),
        }
    }
}
