use yew::prelude::*;


enum Msg {
    On,
    Off,
}


struct Nav {
    link: ComponentLink<Self>,
    value: i8,
}


impl Component for Nav {
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
            Msg::On => {
                self.value += 1;
                true
            },
	        Msg::Off => {
		        self.value -= 1;
		        true
	        },
        }
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }


    fn view(&self) -> Html {
        html! {
	    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
		<a class="navbar-brand" href="#">{ "Navbar" }</a>
		<button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
		</button>
		<div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
		        
                        <li class="nav-item active">
                            <a class="nav-link" href="#">{ "home" }<span class="sr-only"></span></a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "github" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "projects" }</a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}

fn main() {
    yew::start_app::<Nav>();
}
