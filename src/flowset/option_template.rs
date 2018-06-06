use super::OptionTemplateItem;
use error::{Error, ParseResult};
use field::TypeLengthField;
use util::take_u16;

pub const OPTION_FLOWSET_ID: u16 = 1;

#[derive(Debug)]
pub struct OptionTemplate {
    pub flowset_id: u16,
    pub length: u16,
    pub template: OptionTemplateItem,
}

impl OptionTemplate {
    pub fn new(
        flowset_id: u16,
        length: u16,
        template_id: u16,
        scope_len: u16,
        opt_len: u16,
        scopes: Vec<TypeLengthField>,
        options: Vec<TypeLengthField>,
    ) -> OptionTemplate {
        let template =
            OptionTemplateItem::new(template_id, scope_len / 4, opt_len / 4, scopes, options);

        OptionTemplate {
            flowset_id: flowset_id,
            length: length,
            template: template,
        }
    }

    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        let (rest, flowset_id) = take_u16(&data).unwrap();

        if flowset_id == OPTION_FLOWSET_ID {
            let (rest, length) = take_u16(&rest).unwrap();
            let (rest, template_id) = take_u16(&rest).unwrap();
            let (rest, scope_len) = take_u16(&rest).unwrap();
            let (rest, option_len) = take_u16(&rest).unwrap();
            let (rest, scopes) = TypeLengthField::to_vec((scope_len / 4) as usize, rest).unwrap();
            let (rest, options) = TypeLengthField::to_vec((option_len / 4) as usize, rest).unwrap();

            Ok((
                rest,
                OptionTemplate::new(
                    flowset_id,
                    length,
                    template_id,
                    scope_len,
                    option_len,
                    scopes,
                    options,
                ),
            ))
        } else {
            Err(Error::InvalidLength)
        }
    }
}

#[cfg(test)]
mod test_option_template {
    use super::OptionTemplate;
    use error::ParseResult;
    use flowset::test_data;

    #[test]
    fn test_option_template() {
        let packet_bytes = &test_data::OPTION_DATA[..];

        let option: ParseResult<OptionTemplate> = OptionTemplate::from_bytes(&packet_bytes);
        assert!(option.is_ok());

        let (_rest, option): (&[u8], OptionTemplate) = option.unwrap();
        assert_eq!(option.flowset_id, 1);
        assert_eq!(option.length, 26);
        assert_eq!(option.template.template_id, 4096);
        assert_eq!(option.template.scope_count, 1);
        assert_eq!(option.template.option_count, 3);
    }

}