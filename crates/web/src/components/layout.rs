use yew::prelude::*;

use super::sidebar::Sidebar;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="flex justify-center min-h-screen">
            // Left sidebar
            <div class="w-64 flex-shrink-0 border-r border-gray-800">
                <Sidebar />
            </div>
            // Main content
            <main class="w-[600px] flex-shrink-0 border-r border-gray-800 min-h-screen">
                { props.children.clone() }
            </main>
            // Right sidebar
            <div class="w-80 flex-shrink-0 p-4">
                <div class="bg-gray-900 rounded-2xl p-4">
                    <h2 class="text-xl font-bold text-white mb-4">{ "What's happening" }</h2>
                    <p class="text-gray-500 text-sm">{ "Trending topics would appear here." }</p>
                </div>
            </div>
        </div>
    }
}
