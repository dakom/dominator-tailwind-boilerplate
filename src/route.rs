use crate::page::button::ButtonPage;
use crate::page::echo::EchoPage;
use crate::page::image::ImagePage;
use crate::page::list::ListPage;
use crate::page::profile::ProfilePage;
use crate::page::cipher::CipherPage;
use crate::page::{home::HomePage, not_found::NotFoundPage};
use crate::prelude::*;
use futures_signals::signal::Signal;
use std::fmt::{Debug, Display};
use web_sys::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Home,
    Image,
    Echo,
    Profile(Option<String>),
    Button,
    NotFound,
    List,
    Cipher,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl Route {
    pub fn render() -> Dom {
        html!("div", {
            .child_signal(Self::current_signal().map(move |route| {
                Some(match route {
                    Route::Home => HomePage::new().render(),
                    Route::Image => ImagePage::new().render(),
                    Route::Echo => EchoPage::new().render(),
                    Route::Profile(value) => ProfilePage::new(value).render(),
                    Route::Button => ButtonPage::new().render(),
                    Route::List => ListPage::new().render(),
                    Route::Cipher => CipherPage::new().render(),
                    Route::NotFound => NotFoundPage::new().render(),
                })
            }))
        })
    }
    pub fn current_signal() -> impl Signal<Item = Self> {
        dominator::routing::url()
            .signal_cloned()
            .map(|url| Self::from_url(&url))
    }

    pub fn into_uri(&self) -> String {
        let mut s: String = self.into();
        if !CONFIG.uri_root.is_empty() {
            s = format!("/{}{}", CONFIG.uri_root, s);
        }

        s
    }

    pub fn go_to_url(&self) {
        dominator::routing::go_to_url(&self.into_uri());
    }

    pub fn hard_redirect(&self) {
        let location = web_sys::window().unwrap().location();
        let s: String = self.into_uri();
        location.set_href(&s).unwrap();
    }

    pub fn push_state(&self) {
        let history = web_sys::window().unwrap().history().unwrap();
        let url: String = self.into_uri();
        let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
    }

    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        let paths = url.pathname();
        let mut paths = paths
            .split('/')
            .into_iter()
            .skip(if CONFIG.uri_root.is_empty() { 1 } else { 2 })
            .collect::<Vec<_>>();
        let paths = paths.as_slice();

        match paths {
            [""] => Self::Home,
            ["image"] => Self::Image,
            ["echo"] => Self::Echo,
            ["profile"] => Self::Profile(None),
            ["profile", value] => Self::Profile(Some(value.to_string())),
            ["list"] => Self::List,
            ["button"] => Self::Button,
            ["cipher"] => Self::Cipher,
            _ => Self::NotFound,
        }
    }

    pub fn href(&self) -> String {
        let mut s: String = self.into();
        format!("/{}{}", CONFIG.uri_root, s)
            .replace("//", "/")
    }
}

impl From<Route> for String {
    fn from(route: Route) -> Self {
        (&route).into()
    }
}

impl From<&Route> for String {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home => "/".to_string(),
            Route::Image => "/image".to_string(),
            Route::Echo => "/echo".to_string(),
            Route::List => "/list".to_string(),
            Route::Profile(value) => match value {
                None => "/profile".to_string(),
                Some(value) => format!("/profile/{}", value)
            },
            Route::Cipher => "/cipher".to_string(),
            Route::Button => "/button".to_string(),
            Route::NotFound => "404".to_string(),
        }
    }
}
