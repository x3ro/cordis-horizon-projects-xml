from dataclasses import dataclass, field
from datetime import date, datetime
from enum import Enum
from typing import List, Optional


class AssociationType(Enum):
    ASSOCIATED_PARTNER = "associatedPartner"
    COORDINATOR = "coordinator"
    IS_REGISTERED_BY = "isRegisteredBy"
    PARTICIPANT = "participant"
    THIRD_PARTY = "thirdParty"


@dataclass
class Validation:
    euroSciVoc: bool


@dataclass
class Identifiers:
    grantDoi: Optional[str]
    issn: Optional[str]
    doi: Optional[str]
    isbn: Optional[str]


@dataclass
class Category:
    classification: str
    language: str
    availableLanguages: str
    displayCode: Optional[str]
    code: str
    title: str
    type: str
    description: Optional[str]
    weight: Optional[int]
    order: Optional[int]


@dataclass
class Categories:
    category: List[Category]


@dataclass
class Address:
    street: Optional[str]
    city: Optional[str]
    postalCode: Optional[str]
    country: Optional[str]
    geolocation: Optional[str]
    postBox: Optional[str]
    url: Optional[str]


@dataclass
class Organization:
    rcn: str
    id: str
    legalName: str
    shortName: Optional[str]
    type: AssociationType
    terminated: Optional[bool]
    sme: Optional[bool]  # small-medium enterprise?
    totalCost: Optional[float]
    source: str
    order: Optional[int]
    ecContribution: Optional[float]
    netEcContribution: Optional[float]
    vatNumber: Optional[str]
    address: Address
    availableLanguages: str
    departmentName: Optional[str]
    relations: "Relations"


@dataclass
class WebItem:
    title: Optional[str]
    source: Optional[str]
    type: str
    language: str
    availableLanguages: str
    uri: str
    alternativeText: Optional[str]
    mimetype: Optional[str]
    size: int
    hashValue: Optional[str]
    represents: Optional[str]
    copyright: Optional[str]
    order: Optional[int]
    description: Optional[str]


@dataclass
class Call:
    source: str
    type: str
    rcn: str
    title: str
    identifier: str


@dataclass
class WebLink:
    title: Optional[str]
    source: str
    represents: Optional[str]
    type: str
    language: str
    availableLanguages: str
    id: str
    physUrl: str
    defaultLanguage: str
    archivedDate: Optional[datetime]
    status: Optional[str]


@dataclass
class ResultDetails:
    authors: Optional[str]
    journalNumber: Optional[str]
    journalTitle: Optional[str]
    publishedPages: Optional[str]
    publishedYear: Optional[str]
    publisher: Optional[str]
    iprAwarded: Optional[str]
    iprNumber: Optional[str]
    iprDate: Optional[datetime]
    iprPrefix: Optional[str]


@dataclass
class Result:
    source: str
    type: str
    availableLanguages: str
    rcn: str
    id: str
    title: Optional[str]
    description: Optional[str]
    teaser: Optional[str]
    sourceUpdateDate: Optional[datetime]
    contentUpdateDate: Optional[datetime]
    relations: "Relations"
    identifiers: Optional[Identifiers]
    details: Optional[ResultDetails]


@dataclass
class Article:
    source: str
    type: str
    availableLanguages: str
    rcn: str
    id: Optional[str]
    title: str
    contentUpdateDate: Optional[datetime]
    archivedDate: Optional[datetime]
    relations: "Relations"
    teaser: Optional[str]
    order: Optional[str]
    section: Optional[str]


@dataclass
class Programmes:
    programme: List["Programme"]


@dataclass
class Programme:
    source: Optional[str]
    uniqueProgrammePart: Optional[bool]
    type: Optional[str]
    availableLanguages: Optional[str]
    rcn: str
    id: Optional[str]
    code: str
    frameworkProgramme: Optional[str]
    pga: Optional[str]
    title: Optional[str]
    parent: Optional[Programmes]
    relations: Optional["Relations"]


@dataclass
class Associatons:
    organization: List[Organization] = field(default_factory=list)
    article: List[Article] = field(default_factory=list)
    call: List[Call] = field(default_factory=list)
    programme: List[Programme] = field(default_factory=list)
    result: List[Result] = field(default_factory=list)
    webItem: List[WebItem] = field(default_factory=list)
    webLink: List[WebLink] = field(default_factory=list)


@dataclass
class Regions:
    region: List["Region"]


@dataclass
class Region:
    type: Optional[str]
    name: Optional[str]
    rcn: str
    nutsCode: Optional[str]
    parents: Optional[Regions]
    euCode: Optional[str]
    isoCode: Optional[str]


@dataclass
class Relations:
    categories: Optional[Categories]
    associations: Optional[Associatons]
    regions: Optional[List[Regions]]


@dataclass
class Project:
    rcn: str
    id: str
    acronym: str
    teaser: str
    objective: str
    title: str
    keywords: Optional[str]
    identifiers: Optional[Identifiers]
    availableLanguages: str
    duration: float
    language: str
    validation: Optional[Validation]
    totalCost: float
    ecMaxContribution: float
    startDate: date
    endDate: date
    ecSignatureDate: Optional[date]
    status: str
    sourceUpdateDate: datetime
    contentCreationDate: datetime
    contentUpdateDate: datetime
    lastUpdateDate: datetime
    terminationDate: Optional[datetime]
    relations: Relations
