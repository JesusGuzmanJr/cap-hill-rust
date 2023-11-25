use {crate::components::*, leptos::*, leptos_meta::*};

// TODO: add library code here

#[component]
pub fn Library() -> impl IntoView {
    view! {
        <Meta name="description" content="Discover and checkout books related to Rust/CS/entrepreneurship as part of Cap Hill Rust Meetup group." />
        <header>
            <h1>{crate::ORG_NAME}" - Library"</h1>
            <p>"Our library contains books about Rust, computer science, and entrepreneurship."</p>
            <Nav/>
        </header>

        <main>
            <p>"Our library is open to anybody who comes to our meetups. Use the search bar to check out, renew or return books."
            </p>
            <form>
                <fieldset id="forms__input">
                <legend>Catalog Search</legend>
                <p>
                    <label for="input__text">Search by title or author</label>
                    <input id="input__text" type="text" placeholder="Rust Programming Language"/>
                </p>
                <table>
                    <thead>
                    <tr>
                        <th>Book</th>
                        <th>Details</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                        <figure>
                            <img alt="The Rust Programming Book" src="/images/foo.jpg" />
                            <figcaption>The Rust Programming Book</figcaption>
                        </figure>
                        </td>
                        <td>
                        <dl>
                            <dt>Author(s)</dt>
                            <dd>Steve Klabnik</dd>
                            <dd>Carol Nichols</dd>
                            <dt>Publisher</dt>
                            <dd>No Starch Press</dd>
                            <dt>Published</dt>
                            <dd>February 28, 2023</dd>
                            <dt>Status</dt>
                            <dd>Checked out until March 2023</dd>
                            <dt>ISBN-13</dt>
                            <dd>978-1718503106</dd>
                        </dl>
                        <details>
                            <summary>Summary</summary>
                            <p>"Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptas, modi iure! Incidunt, dolorum sit?
                            Dolorum
                            cumque omnis accusantium doloremque nihil est perferendis voluptas delectus, quis aperiam blanditiis
                            deleniti modi
                            at. Lorem ipsum dolor sit amet consectetur adipisicing elit. Vel, perspiciatis, vero accusantium sed
                            dicta
                            exercitationem iure praesentium nobis esse ullam sunt cum blanditiis! Neque similique corporis animi
                            voluptatibus et
                            modi."</p>
                        </details>
                        // <button disabled>Checked out</button>
                        <button>Place Hold</button>
                        <button>Return</button>
                        </td>
                    </tr>
                    </tbody>
                </table>
                </fieldset>
                <p><a href="#top">[Top]</a></p>
            </form>
        </main>
         <Footer/>
    }
}
