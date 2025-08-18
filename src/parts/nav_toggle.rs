use crate::prelude::*;

#[allow(unused_variables)]

impl<T> Site<T> {
    pub fn construct_nav_toggle(&mut self) -> (String, String) {
        self.declare_css(
            "nav_toggle",
            r##"
                /* ===== NAV TOGGLE BUTTON ===== */

                /* Hide checkbox */
                #nav-toggle-checkbox {
                    display: none;
                }

                /* Nav toggle button (hidden by default) */
                #nav-toggle-button {
                    --checkbox-width: 35px;
                    --padding: 10px;
                    cursor: pointer;
                    z-index: 2000;
                    display: none;
                    place-items: center;
                    justify-content: center;
                    /*height: calc(var(--checkbox-width) + var(--padding) * 2);
                    width: calc(var(--checkbox-width) + var(--padding) * 2);*/
                    height: 35px;
                    width: 35px;
                    padding: var(--padding);
                    touch-action: manipulation;
                    user-select: none;
                    -webkit-tap-highlight-color: transparent;
                }

                /* Nav toggle symbol lines */
                .nav-toggle-symbol {
                    position: relative;
                    width: var(--checkbox-width);
                    height: 3px;
                    background: #B7472A;
                    display: block;
                    transition: all 0.2s ease;
                    user-select: none;
                    
                    &::before,
                    &::after {
                        content: "";
                        position: absolute;
                        left: 0;
                        width: var(--checkbox-width);
                        height: 3px;
                        background: #B7472A;
                        transition: all 0.2s ease;
                        user-select: none;
                    }
                    
                    &::before {
                        top: -10px;
                    }
                    
                    &::after {
                        top: 10px;
                    }
                    
                }

                /* Transform to X when checked */
                #nav-toggle-checkbox:checked ~ #nav-toggle-button {
                    
                    .nav-toggle-symbol {
                        background: transparent;
                    }
                    .nav-toggle-symbol::before {
                        top: 0;
                        transform: rotate(135deg);
                    }
                    .nav-toggle-symbol::after {
                        top: 0;
                        transform: rotate(-135deg);
                    }

                }

                /* ===== NAV TOGGLE BUTTON MOBILE ===== */
                @media screen and (max-width: [mobile]) {
                    
                    /* Show nav toggle on mobile */
                    #nav-toggle-button {
                        display: flex;
                    }
                    
                }
            "##,
        );

        (
            r##"
            
            <input id="nav-toggle-checkbox" type="checkbox">

            "##
            .to_owned(),
            r##"

            <label id="nav-toggle-button" for="nav-toggle-checkbox">
                <span class="nav-toggle-symbol"></span>
            </label>
            
            "##
            .to_owned(),
        )
    }
}
