/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::SVGGraphicsElementBinding;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::Root;
use dom::bindings::str::DOMString;
use dom::document::Document;
use dom::node::Node;
use dom::svgelement::SVGElement;
use dom::virtualmethods::VirtualMethods;
use html5ever_atoms::LocalName;
use style::element_state::ElementState;

#[dom_struct]
pub struct SVGGraphicsElement {
    svgelement: SVGElement,
}

impl SVGGraphicsElement {
    pub fn new_inherited(tag_name: LocalName, prefix: Option<DOMString>,
                         document: &Document) -> SVGGraphicsElement {
        SVGGraphicsElement::new_inherited_with_state(ElementState::empty(), tag_name, prefix, document)
    }

    pub fn new_inherited_with_state(state: ElementState, tag_name: LocalName,
                                    prefix: Option<DOMString>, document: &Document)
                                    -> SVGGraphicsElement {
        SVGGraphicsElement {
            svgelement:
                SVGElement::new_inherited_with_state(state, tag_name, prefix, document),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(local_name: LocalName, prefix: Option<DOMString>, document: &Document) -> Root<SVGGraphicsElement> {
        Node::reflect_node(box SVGGraphicsElement::new_inherited(local_name, prefix, document),
                           document,
                           SVGGraphicsElementBinding::Wrap)
    }
}

impl VirtualMethods for SVGGraphicsElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<SVGElement>() as &VirtualMethods)
    }
}