use std::collections::HashMap;

use component::Component;
use component::parse_component;
use property::Property;

use error::*;

pub struct Vcard(Component);

impl Vcard {

    /// Parse a string to a Vcard object
    ///
    /// Returns an error if the parsed text is not a Vcard (that means that an error is returned
    /// also if this is a valid icalendar!)
    ///
    pub fn build(s: &str) -> Result<Vcard> {
        parse_component(s)
            .and_then(|c| if c.name == "VCARD" {
                Ok(Vcard(c))
            } else {
                let kind = VObjectErrorKind::NotAVCard(s.to_owned());
                Err(VObjectError::from_kind(kind))
            })
    }

    pub fn adr(&self) -> Vec<Adr> {
        unimplemented!()
    }

    pub fn anniversary(&self) -> Option<Anniversary> {
        unimplemented!()
    }

    pub fn bday(&self) -> Option<BDay> {
        unimplemented!()
    }

    pub fn categories(&self) -> Vec<Category> {
        unimplemented!()
    }

    pub fn clientpidmap(&self) -> Option<ClientPidMap> {
        unimplemented!()
    }

    pub fn email(&self) -> Vec<Email> {
        unimplemented!()
    }

    /// The 'FN' Attribute of the Vcard.
    ///
    /// # Note
    ///
    /// The property _must_ be present due to Section 6.2.1 of RFC 6350.
    /// Therefor, an empty vector returned can be considered an error, though we do not return a
    /// Result here.
    ///
    /// # Returns
    ///
    /// As the cardinality of this property is 1..*, we return a Vec<String> here.
    ///
    pub fn fullname(&self) -> Vec<FullName> {
        self.0
            .get_all("FN")
            .iter()
            .map(Clone::clone)
            .map(FullName::from)
            .collect()
    }

    pub fn gender(&self) -> Option<Gender> {
        unimplemented!()
    }

    pub fn geo(&self) -> Vec<Geo> {
        unimplemented!()
    }

    pub fn impp(&self) -> Vec<IMPP> {
        unimplemented!()
    }

    pub fn key(&self) -> Vec<Key> {
        unimplemented!()
    }

    pub fn lang(&self) -> Vec<Lang> {
        unimplemented!()
    }

    pub fn logo(&self) -> Vec<Logo> {
        unimplemented!()
    }

    pub fn member(&self) -> Vec<Member> {
        unimplemented!()
    }

    pub fn name(&self) -> Result<Name> {
        self.0
            .get_only("N")
            .cloned()
            .map(Name::from)
            .ok_or(VObjectError::from_kind(VObjectErrorKind::NameNotFound))
    }

    pub fn nickname(&self) -> Vec<NickName> {
        self.0
            .get_all("NICKNAME")
            .iter()
            .map(Clone::clone)
            .map(NickName::from)
            .collect()
    }

    pub fn note(&self) -> Vec<Note> {
        unimplemented!()
    }

    pub fn org(&self) -> Vec<Organization> {
        unimplemented!()
    }

    pub fn photo(&self) -> Vec<Photo> {
        unimplemented!()
    }

    pub fn proid(&self) -> Option<Proid> {
        unimplemented!()
    }

    pub fn related(&self) -> Vec<Related> {
        unimplemented!()
    }

    pub fn rev(&self) -> Option<Rev> {
        unimplemented!()
    }

    pub fn role(&self) -> Vec<Title> {
        unimplemented!()
    }

    pub fn sound(&self) -> Vec<Sound> {
        unimplemented!()
    }

    pub fn tel(&self) -> Vec<Tel> {
        unimplemented!()
    }

    pub fn title(&self) -> Vec<Title> {
        unimplemented!()
    }

    pub fn tz(&self) -> Vec<Tz> {
        unimplemented!()
    }

    pub fn uid(&self) -> Option<Uid> {
        unimplemented!()
    }

    pub fn url(&self) -> Vec<Url> {
        unimplemented!()
    }

    pub fn version(&self) -> Result<Version> {
        self.0
            .get_only("VERSION")
            .map(|p| p.raw_value.clone())
            .ok_or(VObjectError::from_kind(VObjectErrorKind::VersionNotFound))
    }

}

pub type Version = String;

pub type Parameters = HashMap<String, String>;

macro_rules! create_data_type {
    ( $name:ident ) => {
        pub struct $name(String, Parameters);

        impl $name {
            fn new(raw: String, params: Parameters) -> $name {
                $name(raw, params)
            }
        }

        impl From<Property> for $name {
            fn from(p: Property) -> $name {
                $name::new(p.raw_value, p.params)
            }
        }
    }
}

create_data_type!(Adr);
create_data_type!(Anniversary);
create_data_type!(BDay);
create_data_type!(Category);
create_data_type!(ClientPidMap);
create_data_type!(Email);
create_data_type!(FullName);
create_data_type!(Gender);
create_data_type!(Geo);
create_data_type!(IMPP);
create_data_type!(Key);
create_data_type!(Lang);
create_data_type!(Logo);
create_data_type!(Member);
create_data_type!(Name);
create_data_type!(NickName);
create_data_type!(Note);
create_data_type!(Organization);
create_data_type!(PhoneNumber);
create_data_type!(Photo);
create_data_type!(Proid);
create_data_type!(Related);
create_data_type!(Rev);
create_data_type!(Sound);
create_data_type!(Tel);
create_data_type!(Title);
create_data_type!(Tz);
create_data_type!(Uid);
create_data_type!(Url);

/// A Name type
///
/// offers functionality to get firstname, middlenames and lastname.
///
/// The parsing behaviour is implemented in a way that splits at whitespace, following these rules:
///
/// * If there is only one element after splitting, this is considered the lastname
/// * If there are two elements, this is firstname and lastname
/// * If there are more than two elements, firstname and lastname are the first and last elements
/// respectively, all others are middlenames.
///
impl Name {

    pub fn plain(&self) -> String {
        self.0.clone()
    }

    pub fn surname(&self) -> Option<String> {
        self.0.split(";").nth(0).map(String::from)
    }

    pub fn given_name(&self) -> Option<String> {
        self.0.split(";").nth(1).map(String::from)
    }

    pub fn additional_names(&self) -> Option<String> {
        self.0.split(";").nth(2).map(String::from)
    }

    pub fn honorific_prefixes(&self) -> Option<String> {
        self.0.split(";").nth(3).map(String::from)
    }

    pub fn honorific_suffixes(&self) -> Option<String> {
        self.0.split(";").nth(4).map(String::from)
    }

    /// Alias for Name::surname()
    #[inline]
    pub fn family_name(&self) -> Option<String> {
        self.surname()
    }

}

