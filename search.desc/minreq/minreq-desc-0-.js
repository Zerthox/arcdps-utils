searchState.loadedDescShard("minreq", 0, "Minreq\nToSocketAddrs did not resolve to an address.\nThe provided proxy information was not properly formatted. …\nThe provided credentials were rejected by the proxy server.\nThe CONNECT method\nA custom method, use with care: the string will be …\nThe DELETE method\nRepresents an error while sending, receiving, or parsing …\nThe GET method\nThe HEAD method\nThe response contains headers whose total size surpasses …\nTried to send a secure request (ie. the url started with …\nThe response redirections caused an infinite redirection …\nThe provided credentials were rejected by the proxy server.\nThe response body contains invalid UTF-8, so the <code>as_str()</code> …\nThe response contained invalid UTF-8 where it should be …\nRan into an IO problem while loading the response.\nThe chunk did not end after reading the previously read …\nCouldn’t parse the incoming chunk’s length while …\nCouldn’t parse the <code>Content-Length</code> header’s value as an …\nAn HTTP request method.\nThe OPTIONS method\nThis is a special error case, one that should never be …\nThe PATCH method\nThe POST method\nThe provided proxy credentials were malformed.\nThe provided url contained a domain that has non-ASCII …\nThe provided url contained a domain that has non-ASCII …\nThe PUT method\nThe response was a redirection, but the <code>Location</code> header is …\nAn HTTP request.\nAn HTTP response.\nAn HTTP response, which is loaded lazily.\nRan into a Serde error.\nThe response’s status line length surpasses …\nFollowed <code>max_redirections</code> redirections, won’t follow any …\nThe TRACE method\nA URL type for requests.\nReturns a reference to the contained bytes of the body. If …\nReturns the body as an <code>&amp;str</code>.\nAlias for Request::new with <code>method</code> set to Method::Connect.\nAlias for Request::new with <code>method</code> set to Method::Delete.\nFormats the Method to the form in the HTTP request, ie. …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nAlias for Request::new with <code>method</code> set to Method::Get.\nAlias for Request::new with <code>method</code> set to Method::Head.\nThe headers of the response. The header field names (the …\nThe headers of the response. The header field names (the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTurns the <code>Response</code> into the inner <code>Vec&lt;u8&gt;</code>, the bytes that …\nConverts JSON body to a <code>struct</code> using Serde.\nCreates a new HTTP <code>Request</code>.\nAlias for Request::new with <code>method</code> set to Method::Options.\nAlias for Request::new with <code>method</code> set to Method::Patch.\nAlias for Request::new with <code>method</code> set to Method::Post.\nAlias for Request::new with <code>method</code> set to Method::Put.\nThe reason phrase of the response, eg. “Not Found”.\nThe reason phrase of the response, eg. “Not Found”.\nSends this request to the host.\nSends this request to the host, loaded lazily.\nThe status code of the response, eg. 404.\nThe status code of the response, eg. 404.\nAlias for Request::new with <code>method</code> set to Method::Trace.\nThe URL of the resource returned in this response. May …\nThe URL of the resource returned in this response. May …\nSets the request body.\nAdds a header to the request this is called on. Use this …\nAdd headers to the request this is called on. Use this …\nConverts given argument to JSON and sets it as body.\nSets the maximum size of all the headers this request will …\nSets the max redirects we follow until giving up. 100 by …\nSets the maximum length of the status line this request …\nAdds given key and value as query parameter to request url …\nSets the request timeout in seconds.")