use iced::{widget::text, Element};




fn main() -> iced::Result
{
    #[derive(Debug, Clone)]
    enum Message {}
    fn update(amogus: &mut u64, message: Message)
    {

    }

    fn view(amougs: &u64) -> Element<Message>
    {   
        text("gadse nya").into()
    }
    iced::run("seven segment test", update, view)
}