use nom::{
    bytes::complete::{tag, take_while},
    AsChar, IResult,
};

// https://datatracker.ietf.org/doc/html/rfc2782
#[derive(Debug, Clone)]
pub struct ServiceQuery {
    domain: Domain,
    proto: Proto,
    service: Service,
}

#[derive(Debug, Clone)]
pub struct Domain(String);

#[derive(Debug, Clone)]
pub struct Service(String);

#[derive(Debug, Clone)]
pub struct Proto(String);

impl Domain {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, domain) = take_while(AsChar::is_alphanum)(input)?;
        Ok((rest, Self(domain.to_string())))
    }
}

impl Service {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = tag("_")(input)?;
        let (rest, service) = take_while(AsChar::is_alphanum)(rest)?;
        Ok((rest, Self(service.to_string())))
    }
}

impl Proto {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = tag("_")(input)?;
        let (rest, proto) = take_while(AsChar::is_alphanum)(rest)?;
        Ok((rest, Self(proto.to_string())))
    }
}

impl ServiceQuery {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, service) = Service::parse(input)?;
        let (rest, _) = tag(".")(rest)?;
        let (rest, proto) = Proto::parse(rest)?;
        let (rest, _) = tag(".")(rest)?;
        let (rest, domain) = Domain::parse(rest)?;

        Ok((
            rest,
            ServiceQuery {
                domain,
                proto,
                service,
            },
        ))
    }

    pub fn to_string(&self) -> String {
        format!("_{}._{}.{}", self.service.0, self.proto.0, self.domain.0)
    }

    pub fn decode(input: &str) -> Option<Self> {
        Self::parse(input)
            .map(|(_, service_query)| service_query)
            .ok()
    }
}
