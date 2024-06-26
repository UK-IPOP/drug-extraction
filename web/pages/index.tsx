import Head from "next/head";
import { Container, Link, Text, Image, Spacer, Input, Button, PressEvent } from "@nextui-org/react";
import { useState } from "react";
var distance = require("jaro-winkler");
var levenshtein = require("damerau-levenshtein");

export default function Home() {
	const [term1, setTerm1] = useState<string>("cocaine");
	const [term2, setTerm2] = useState<string>("cociane");
	const [similarity, setSimilarity] = useState<number>(0.967);
	const [edits, setEdits] = useState<number>(1);
	const [is_match, setMatch] = useState<boolean>(true);

	function compare(_: PressEvent) {
		const s: number = distance(term1, term2);
		const e: number = levenshtein(term1, term2).steps;
		setSimilarity(s);
		setEdits(e);
		console.log(edits, similarity);
		if (e === 0 || e === 1) {
			setMatch(true);
		} else if (e === 2 && s >= 0.97) {
			setMatch(true);
		} else {
			setMatch(false);
		}
	}

	return (
		<>
			<Head>
				<title>Create Next App</title>
				<meta name="description" content="Generated by create next app" />
				<meta name="viewport" content="width=device-width, initial-scale=1" />
				<link rel="icon" href="/favicon.ico" />
			</Head>
			<main>
				<Container display="flex" direction="column" justify="center" alignItems="center">
					<Text h1 color="primary">
						UK IPOP Fuzzy Drug Searcher
					</Text>
					<Spacer y={1} />
					<Image src="/imgs/logo.png" alt="logo" width={300} height={300} objectFit="cover" />
					<Container display="flex" direction="row" justify="center" alignItems="center">
						<Text>
							This tool allows you to compare two terms (drugs) and see how similar they are. It
							ideally should be used as an explorative tool to identify potential search terms to
							use in fuzzy search matching. We show information on whether this would be considered
							a match in our <a href="https://github.com/UK-IPOP/drug-extraction">CLI tool</a> using
							the same algorithmic criteria:
						</Text>
						<ol>
							<li>
								0 or 1 edits: <strong>MATCH</strong>
							</li>
							<li>
								2 edits and greater than or equal to 0.97 similarity: <strong>MATCH</strong>
							</li>
							<li>
								2 edits and less than 0.97 similarity: <strong>NO MATCH</strong>
							</li>
							<li>
								3 or more edits: <strong>NO MATCH</strong>
							</li>
						</ol>
					</Container>
					<Spacer y={1} />
					<Container display="flex" direction="row" justify="center" alignItems="center">
						<Input
							underlined
							labelPlaceholder="Term 1"
							color="primary"
							value={term1}
							onChange={(e) => {
								e.preventDefault();
								setTerm1(e.target.value);
							}}
						/>
						<Spacer x={1} />
						<Input
							underlined
							labelPlaceholder="Term 2"
							color="primary"
							value={term2}
							onChange={(e) => {
								e.preventDefault();
								setTerm2(e.target.value);
							}}
						/>
					</Container>
					<Spacer y={1} />
					<Button size="lg" color="primary" onPress={compare}>
						Compare
					</Button>
					<Spacer y={1} />
					<Text h5>
						The terms are {(similarity * 100).toPrecision(3)}% similar
						<br />
						The terms are {edits} edits away
					</Text>
					{is_match ? (
						<Text h5 color="green">
							MATCH
						</Text>
					) : (
						<Text h5 color="red">
							NO MATCH
						</Text>
					)}
					<Spacer y={2} />
					<Text h5>
						Maintained by{" "}
						<Link href="https://pharmacy.uky.edu/office-research-operations/cornerstones/research-centers/ipop">
							UK IPOP
						</Link>
					</Text>
				</Container>
			</main>
		</>
	);
}
