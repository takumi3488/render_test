Rails.application.routes.draw do
  get 'foos/render_test/sym', to: 'foos#render_test_sym'
  get 'foos/render_test/str', to: 'foos#render_test_str'
  resources :foos
  # Define your application routes per the DSL in https://guides.rubyonrails.org/routing.html

  # Defines the root path route ("/")
  # root "articles#index"
end
