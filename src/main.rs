use yew::prelude::*;


enum Msg {
    AddOne,
}


struct Model {
    link: ComponentLink<Self>,
    value: i64,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();


    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }


    fn view(&self) -> Html {
        html! {
	    <nav class="navbar navbar-expand-lg navbar-light bg-light">
		<a class="navbar-brand" href="#">{ "Navbar" }</a>
		<button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
		</button>
		<div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item active">
                            <a class="nav-link" href="#">{ "Home" }<span class="sr-only"></span></a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "Features" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "Pricing" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link disabled" href="#">{ "Disabled" }</a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
