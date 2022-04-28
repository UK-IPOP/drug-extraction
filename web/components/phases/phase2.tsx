import { Button, Card, Col, Container, Grid, Row, Spacer, Text } from "@nextui-org/react";
import * as React from "react";
import { SingleValue } from "react-select";
import Selector from "../selector";
import { Phase2Options } from "../types";
import styles from '../../styles/Home.module.css'
import ArrowForwardIcon from '@mui/icons-material/ArrowForward';

interface Phase2Props {
    headerOptions: string[];
    dataHandler: (data: Phase2Options) => void

};

const Phase2Component = ({ headerOptions, dataHandler }: Phase2Props): JSX.Element => {
    const [idColumn, setIdColumn] = React.useState<number>(-1);
    const [targetCol, setTargetCol] = React.useState<number>(-1);
    const [searchType, setSearchType] = React.useState<string>('simple');
    const [filterType, setFilterType] = React.useState<string>('edits');

    const handleIDSelect = (e: SingleValue<{ value: number; label: string }>) => {
        if (e) {
            setIdColumn(e.value);
        } else {
            setIdColumn(-1);
        }
    };
    const handleTargetSelect = (e: SingleValue<{ value: number; label: string }>) => {
        if (e) {
            setTargetCol(e.value);
        }
    };

    return (
        <Container gap={3} justify="center">
            <Row justify="center">
                <Text h2 className={styles.subtitle}>Now select some options for the search process:</Text>
            </Row>

            <Row gap={2} justify="center">
                <Col span={6}>
                    <Text h5>ID Column:</Text>
                    <Selector
                        optionsList={headerOptions}
                        placeholder="Select an ID column"
                        onSelected={handleIDSelect}
                        clearable={true}
                    />
                </Col>
                <Col span={6}>
                    <Text h5>Target Column:</Text>
                    <Selector
                        optionsList={headerOptions}
                        placeholder="Select a Target column"
                        onSelected={handleTargetSelect}
                    />
                </Col>
            </Row>

            <Row gap={2} justify="center">
                <Col span={6}>
                    <Text h5>Search Type: (simple/custom OR drug/RxNorm)</Text>
                    <Selector
                        optionsList={['simple', 'drug']}
                        placeholder="simple"
                        onSelected={(e) => e ? setSearchType(e.label) : null}
                    />
                </Col>
                <Col span={6}>
                    <Text h5>Limiter/Filter Type:</Text>
                    <Selector
                        optionsList={['edits', 'similarity']}
                        placeholder="edits"
                        onSelected={(e) => e ? setFilterType(e.label) : null}
                    />
                </Col>
            </Row>

            <Spacer y={2} />
            <Row justify="center">
                <Col offset={2} span={4}>
                    <Button rounded onClick={() => {
                        dataHandler({
                            idColumnIndex: idColumn,
                            targetColumnIndex: targetCol,
                            algorithm: "Levenshtein",
                            searchType: searchType,
                            filterType: filterType,
                        });
                    }}><ArrowForwardIcon /></Button>
                </Col>
            </Row >
        </Container >
    );

};

export default Phase2Component;